use jsonschema::JSONSchema;
use serde_json::{Error, Map, Value};
use std::collections::HashMap;
use thiserror::Error;

#[derive(Default)]
pub struct Mask {
    pub properties: HashMap<String, Option<Mask>>,
}

pub struct ValidJsonSchema(Value);

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("serde json could not parse the invalid json")]
    InvalidJson(#[from] Error),
    #[error("the provided json was valid, but it wasn't a valid json schema")]
    InvalidJsonSchema(String),
}

impl ValidJsonSchema {
    pub fn new(schema: Value) -> Result<Self, ParseError> {
        // JSONSchema will validate that the nested portion of a schema is valid, but if the root
        // isn't then it will accept it anyway. This violates our invariants, so we need to check
        // them explicitly at the root.
        if !schema.is_object()
            || !schema.as_object().unwrap().contains_key("type")
            || !schema.as_object().unwrap().get("type").unwrap().is_string()
        {
            return Err(ParseError::InvalidJsonSchema(
                "Invalid JSON Schema object".to_string(),
            ));
        }

        match JSONSchema::options().compile(&schema) {
            Ok(_) => Ok(ValidJsonSchema(schema)),
            Err(error) => Err(ParseError::InvalidJsonSchema(error.to_string())),
        }
    }
}

pub fn from_str(json: &str) -> Result<Mask, ParseError> {
    Ok(Mask::from(&ValidJsonSchema::new(serde_json::from_str::<
        Value,
    >(json)?)?))
}

pub fn from_reader<R>(reader: R) -> Result<Mask, ParseError>
where
    R: std::io::Read,
{
    Ok(Mask::from(&ValidJsonSchema::new(
        serde_json::from_reader::<R, Value>(reader)?,
    )?))
}

fn parse_schema_node(mask: &mut Mask, schema: &Value) {
    // unwrap is safe, because we only recurse for objects, and we validate that the provided json
    // conforms to "json schema" schema (not a typo).
    if let Some(properties) = schema.as_object().unwrap().get("properties") {
        if let Some(properties) = properties.as_object() {
            for (key, child) in properties {
                let child_object = child.as_object().unwrap().get("type");

                if child_object.is_some() && child_object.unwrap() == "object" {
                    let mut child_mask = Mask::default();
                    parse_schema_node(&mut child_mask, child);

                    mask.properties.insert(key.clone(), Some(child_mask));
                } else {
                    mask.properties.insert(key.clone(), None);
                }
            }
        }
    }
}

impl From<&ValidJsonSchema> for Mask {
    fn from(value: &ValidJsonSchema) -> Self {
        let mut mask = Mask::default();

        if value
            .0
            .as_object()
            .unwrap()
            .get("type")
            .unwrap()
            .as_str()
            .unwrap()
            == "object"
        {
            parse_schema_node(&mut mask, &value.0);
        }

        mask
    }
}

pub struct JsonMasker {
    mask: Mask,
}

impl JsonMasker {
    pub fn new(mask: Mask) -> Self {
        JsonMasker { mask }
    }

    pub fn mask(&self, object: &mut Value) {
        if let Some(unwrapped_object) = object.as_object_mut() {
            JsonMasker::mask_object(unwrapped_object, &self.mask)
        }
    }

    fn mask_object(object: &mut Map<String, Value>, mask_node: &Mask) {
        object.retain(|key, value| match mask_node.properties.get(key) {
            None => false,
            Some(mask_child_node) => {
                if let Some(node) = value.as_object_mut() {
                    if let Some(mask_child_node) = mask_child_node {
                        JsonMasker::mask_object(node, mask_child_node)
                    }
                }

                true
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::str::FromStr;
    use uuid::{uuid, Uuid};

    fn get_masker(schema: &str) -> JsonMasker {
        JsonMasker::new(Mask::from(&get_valid_schema(schema).unwrap()))
    }

    fn get_valid_schema(schema: &str) -> Result<ValidJsonSchema, ParseError> {
        ValidJsonSchema::new(serde_json::from_str(schema).unwrap())
    }

    const NONCE: u64 = 12345;
    const VM_ID: Uuid = uuid!("19e656a1-b9ca-4344-88c9-ef0a6b5999e5");
    const FOO: &str = "foo-value";
    const BAR: &str = "bar-value";
    const CREATED_ON: &str = "2023-07-28 17:59:14Z";
    const EXPIRES_ON: &str = "2023-07-28 20:59:14Z";

    fn get_metadata_json() -> Value {
        json!({
            "nonce": 12345,
            "vmId": VM_ID
        })
    }

    fn get_foobar_json() -> Value {
        json!({
            "foo": FOO,
            "bar": BAR
        })
    }

    fn get_mixed_json() -> Value {
        json!({
            "nonce": NONCE,
            "foo": FOO
        })
    }

    #[test]
    // There are lots of unwraps in the the masker parsing because it expects a valid schema.
    // This test ensures that the valid schema wrapper is holding these invariants correctly.
    pub fn bad_schema_no_panic() {
        assert!(get_valid_schema(INVALID_SCHEMA_OBJECT).is_err());
        assert!(get_valid_schema(INVALID_NESTED_SCHEMA).is_err());
        assert!(get_valid_schema(INVALID_SCHEMA_NULL_TYPE).is_err());
        assert!(get_valid_schema(INVALID_SCHEMA_NULL_PROPERTIES).is_err());
        assert!(get_valid_schema(RANDOM_JSON).is_err());
    }

    #[test]
    // Schema validator only checks that provided fields are valid, but missing information is
    // allowed.
    pub fn schema_missing_expected_fields_no_panic() {
        get_masker(INVALID_SCHEMA_EMPTY_PROPERTIES);
        get_masker(INVALID_SCHEMA_NO_TYPE);
        get_masker(INVALID_SCHEMA_NO_PROPERTIES);
    }

    #[test]
    pub fn mask_json_simple_schema_exact_match() {
        let mut json = get_metadata_json();

        get_masker(SIMPLE_SCHEMA).mask(&mut json);

        assert_eq!(NONCE, json["nonce"].as_u64().unwrap());
        assert_eq!(
            VM_ID,
            Uuid::from_str(json["vmId"].as_str().unwrap()).unwrap()
        );
    }

    #[test]
    pub fn mask_json_simple_schema_all_filtered() {
        let mut json = get_foobar_json();

        get_masker(SIMPLE_SCHEMA).mask(&mut json);

        assert!(json.get("foo").is_none());
        assert!(json.get("bar").is_none());
    }

    #[test]
    pub fn mask_json_simple_schema_partially_filtered() {
        let mut json = get_mixed_json();

        get_masker(SIMPLE_SCHEMA).mask(&mut json);

        assert_eq!(NONCE, json["nonce"].as_u64().unwrap());
        assert!(json.get("foo").is_none());
    }

    #[test]
    pub fn mask_json_nested_schema_exact_match() {
        let mut json = get_metadata_json();

        let timestamp = json!({
            "createdOn": CREATED_ON,
            "expiresOn": EXPIRES_ON
        });

        json["timestamp"] = timestamp;

        get_masker(NESTED_SCHEMA).mask(&mut json);

        assert_eq!(NONCE, json["nonce"].as_u64().unwrap());
        assert_eq!(
            VM_ID,
            Uuid::from_str(json["vmId"].as_str().unwrap()).unwrap()
        );
        assert_eq!(CREATED_ON, json["timestamp"]["createdOn"].as_str().unwrap());
        assert_eq!(EXPIRES_ON, json["timestamp"]["expiresOn"].as_str().unwrap());
    }

    #[test]
    pub fn mask_json_nested_schema_all_filtered() {
        let mut json = get_foobar_json();

        let nested_object = json!({
            "foo": FOO,
            "bar": BAR
        });

        json["foobar"] = nested_object;

        get_masker(NESTED_SCHEMA).mask(&mut json);

        assert!(json.get("foo").is_none());
        assert!(json.get("bar").is_none());
        assert!(json.get("foobar").is_none());
    }

    #[test]
    pub fn mask_json_nested_schema_partially_filtered() {
        let mut json = get_mixed_json();
        let nested_object = json!({
            "createdOn": CREATED_ON,
            "bar": BAR
        });

        json["timestamp"] = nested_object;

        get_masker(NESTED_SCHEMA).mask(&mut json);

        assert_eq!(NONCE, json["nonce"].as_u64().unwrap());
        assert!(json.get("foo").is_none());
        assert!(json.get("timestamp").unwrap().is_object());
        assert_eq!(CREATED_ON, json["timestamp"]["createdOn"].as_str().unwrap());
        assert!(json["timestamp"].get("bar").is_none());
    }

    const SIMPLE_SCHEMA: &str = r#"
{
    "$schema": "http://json-schema.org/draft-04/schema",
    "title": "Simple Schema",
    "description": "Arbitrary object for testing",
    "type": "object",
    "properties": {
        "nonce": {
            "type": "string"
        },
        "vmId": {
            "type": "string"
        },
        "foo2": {
            "type": "string"
        }
    }
}
"#;

    const NESTED_SCHEMA: &str = r#"
{
    "$schema": "http://json-schema.org/draft-04/schema",
    "title": "Simple Schema",
    "description": "Arbitrary nested object for testing",
    "type": "object",
    "properties": {
        "nonce": {
            "type": "string"
        },
        "vmId": {
            "type": "string"
        },
        "timestamp": {
            "type": "object",
            "properties": {
                "createdOn": {
                    "type": "string"
                },
                "expiresOn": {
                    "type": "string"
                }
            }
        },
        "foo5": {
            "type": "string"
        }
    }
}
"#;

    const INVALID_SCHEMA_OBJECT: &str = r#"
{
    "$schema": "http://json-schema.org/draft-04/schema",
    "title": "Simple Schema",
    "description": "Arbitrary object for testing",
    "type": "potato",
    "properties": {
        "nonce": {
            "type": "string"
        },
        "vmId": {
            "type": "string"
        },
        "foo2": {
            "type": "string"
        }
    }
}
"#;

    const INVALID_NESTED_SCHEMA: &str = r#"
{
    "$schema": "http://json-schema.org/draft-04/schema",
    "title": "Simple Schema",
    "description": "Arbitrary nested object for testing",
    "type": "object",
    "properties": {
        "nonce": {
            "type": "string"
        },
        "vmId": {
            "type": "string"
        },
        "timestamp": {
            "type": "potato",
            "properties": {
                "createdOn": {
                    "type": "string"
                },
                "expiresOn": {
                    "type": "string"
                }
            }
        },
        "foo5": {
            "type": "string"
        }
    }
}
"#;

    const INVALID_SCHEMA_NO_TYPE: &str = r#"
{
    "$schema": "http://json-schema.org/draft-04/schema",
    "title": "Simple Schema",
    "description": "Arbitrary nested object for testing",
    "type": "object",
    "properties": {
        "nonce": {
        },
        "vmId": {
            "no_type": "string"
        },
        "foo5": {
            "type": "string"
        }
    }
}
"#;

    const INVALID_SCHEMA_NULL_TYPE: &str = r#"
{
    "$schema": "http://json-schema.org/draft-04/schema",
    "title": "Simple Schema",
    "description": "Arbitrary nested object for testing",
    "type": "object",
    "properties": {
        "nonce": {
            "type": null
        }
    }
}
"#;

    const INVALID_SCHEMA_NO_PROPERTIES: &str = r#"
{
    "$schema": "http://json-schema.org/draft-04/schema",
    "title": "Simple Schema",
    "description": "Arbitrary nested object for testing",
    "type": "object",
    "no_properties": {

    }
}
"#;

    const INVALID_SCHEMA_EMPTY_PROPERTIES: &str = r#"
{
    "$schema": "http://json-schema.org/draft-04/schema",
    "title": "Simple Schema",
    "description": "Arbitrary nested object for testing",
    "type": "object",
    "properties": {

    }
}
"#;

    const INVALID_SCHEMA_NULL_PROPERTIES: &str = r#"
{
    "$schema": "http://json-schema.org/draft-04/schema",
    "title": "Simple Schema",
    "description": "Arbitrary nested object for testing",
    "type": "object",
    "properties": null
}
"#;

    const RANDOM_JSON: &str = r#"
{
    "this_is": "valid json but isn't a schema"
}
"#;
}
