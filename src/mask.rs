use crate::serialize::Mask;
use jsonschema::error::ValidationError;
use jsonschema::JSONSchema;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::error::Error;
use serde_json::{Map, Value};
use std::collections::HashMap;
use thiserror::Error;

pub struct ValidJsonSchema(Value);

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("serde json could not parse the invalid json")]
    InvalidJson(#[from] Error),
    #[error("the provided json was valid, but it wasn't a valid json schema")]
    InvalidJsonSchema(String)
    // TODO: is there a better way to handle the lifetime, or must it always be poisonous?
    //InvalidJsonSchema(#[from] ValidationError<'static>),
}

impl ValidJsonSchema {
    pub fn new(schema: Value) -> Result<Self, ParseError> {
        match JSONSchema::compile(&schema) {
            Ok(_) => Ok(ValidJsonSchema { 0: schema }),
            Err(error) => Err(ParseError::InvalidJsonSchema(error.to_string()))
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
    let properties = schema.as_object().unwrap().get("properties").unwrap();

    for (key, child) in properties.as_object().unwrap() {
        let child = child.as_object().unwrap();

        if child.get("type").unwrap() == "object" {
            let mut child_mask = Mask::default();
            parse_schema_node(&mut child_mask, child.get("properties").unwrap());

            mask.properties.insert(key.clone(), Some(child_mask));
        } else {
            mask.properties.insert(key.clone(), None);
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
            self.mask_object(unwrapped_object, &self.mask)
        }
    }

    fn mask_object(&self, object: &mut Map<String, Value>, mask_node: &Mask) {
        object.retain(|key, value| match mask_node.properties.get(key) {
            None => false,
            Some(mask_child_node) => {
                if let Some(node) = value.as_object_mut() {
                    if let Some(mask_child_node) = mask_child_node {
                        self.mask_object(node, mask_child_node)
                    }
                }

                true
            }
        })
    }
}

struct Repro<S>
where
    S: Serializer,
{
    serializer: S,
}

impl<S> Repro<S>
where
    S: Serializer,
{
    fn serialize_bool(self, v: bool) -> Result<S::Ok, S::Error> {
        self.serializer.serialize_bool(v)
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;
    use super::*;

    fn get_masker(schema: &str) -> JsonMasker {
        JsonMasker::new(Mask::from(&ValidJsonSchema::new(serde_json::from_str(schema).unwrap()).unwrap()))
    }

    const ARBITRARY_VALUE: u64 = 12345;

    fn get_metadata_json() -> Value {
        json!({
            "nonce": ARBITRARY_VALUE,
            "vmId": ARBITRARY_VALUE
        })
    }

    fn get_foobar_json() -> Value {
        json!({
            "foo": "foo-value",
            "bar": "bar-value"
        })
    }

    fn get_mixed_json() -> Value {
        json!({
            "nonce": 12345,
            "foo": "foo-value"
        })
    }

    #[test]
    pub fn mask_json_simple_schema_exact_match()
    {
        let mut json = get_metadata_json();

        get_masker(SIMPLE_SCHEMA).mask(&mut json);

        assert_eq!(arbitraryValue, json[L"nonce"].as_string());
        assert_eq!(arbitraryValue, json[L"vmId"].as_string());
    }

    #[test]
    pub fn mask_json_simple_schema_all_filtered()
    {
        let mut json = get_foobar_json();

        get_masker(SIMPLE_SCHEMA).mask(&mut json);
        
        assert_ne!(json.has_string_field(L"foo"));
        assert_ne!(json.has_string_field(L"bar"));
    }

    #[test]
    pub fn mask_json_simple_schema_partially_filtered()
    {
        let mut json = get_mixed_json();

        get_masker(SIMPLE_SCHEMA).mask(&mut json);

        assert_eq!(arbitraryValue, json[L"nonce"].as_string());
        assert_ne!(json.has_string_field(L"foo"));
    }

    #[test]
    pub fn mask_json_nested_schema_exact_match()
    {
        let mut json = get_metadata_json();

        auto timestamp = web::json::value::object();
        timestamp[L"createdOn"] = web::json::value(testValue);
        timestamp[L"expiresOn"] = web::json::value(testValue);
        json[L"timestamp"] = timestamp;

        get_masker(NESTED_SCHEMA).mask(&mut json);

        assert_eq!(arbitraryValue, json[L"nonce"].as_string());
        assert_eq!(arbitraryValue, json[L"vmId"].as_string());
        assert_eq!(arbitraryValue, json[L"timestamp"][L"createdOn"].as_string());
        assert_eq!(arbitraryValue, json[L"timestamp"][L"expiresOn"].as_string());
    }

    #[test]
    pub fn mask_json_nested_schema_all_filtered()
    {
        let mut json = get_foobar_json();

        auto nestedObject = web::json::value::object();
        nestedObject[L"foo"] = web::json::value(testValue);
        nestedObject[L"bar"] = web::json::value(testValue);
        json[L"foobar"] = nestedObject;

        get_masker(NESTED_SCHEMA).mask(&mut json);

        assert_ne!(json.has_string_field(L"foo"));
        assert_ne!(json.has_string_field(L"bar"));
        assert_ne!(json.has_object_field(L"foobar"));
    }

    #[test]
    pub fn mask_json_nested_schema_partially_filtered()
    {
        let mut json = get_mixed_json();
        let nested_object = json!({
            "createdOn": 12345
            "bar": uuid!("67e55044-10b1-426f-9247-bb680e5fe0c8");
        });

        json.as_object_mut().unwrap().insert("timestamp".parse().unwrap(), nested_object);

        get_masker(NESTED_SCHEMA).mask(&mut json);

        assert_eq!(arbitraryValue, json[L"nonce"].as_string());
        assert_ne!(json.has_string_field(L"foo"));
        Assert::IsTrue(json.has_object_field(L"timestamp"));
        assert_eq!(arbitraryValue, json[L"timestamp"][L"createdOn"].as_string());
    }
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
