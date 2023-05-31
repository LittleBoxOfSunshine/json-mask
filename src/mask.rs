use std::collections::HashMap;
use std::path::Component::ParentDir;
use serde_json::{Value, Map};
use serde::{Deserialize, Serialize, Serializer};

// TODO: Reconcile this with the more generic "mask"
pub struct JsonMask {
    name: String,
    properties: HashMap<String, JsonMask>
}

impl From<&str> for JsonMask {
    fn from(value: &str) -> Self {
        todo!()
    }
}

impl From<&String> for JsonMask {
    fn from(value: &String) -> Self {
        todo!()
    }
}

impl From<&Value> for JsonMask {
    fn from(value: &Value) -> Self {
        todo!()
    }
}

pub struct JsonMasker {
    mask: JsonMask
}

impl JsonMasker {
    pub fn new(mask: JsonMask) -> Self {
        JsonMasker { mask }
    }
    
    pub fn mask(&self, object: &mut Value) {
        if let Some(unwrapped_object) = object.as_object_mut() {
            self.mask_object(unwrapped_object, &self.mask)
        }
    }

    fn mask_object(&self, object: &mut Map<String, Value>, mask_node: &JsonMask) {
        object.retain(| key, value | {
            match mask_node.properties.get(key) {
                None => false,
                Some(mask_child_node) => {
                    if let Some(node) = value.as_object_mut() {
                        self.mask_object(node, mask_child_node)
                    }

                    true
                }
            }
        })
    }
}

struct Repro <S>
    where S: Serializer
{
    serializer: S
}

impl<S> Repro<S>
    where S: Serializer
{
    fn serialize_bool(self, v: bool) -> Result<S::Ok, S::Error> {
        self.serializer.serialize_bool(v)
    }
}

#[cfg(test)]
mod tests {
    use serde::Serializer;
    use serde_json::ser::CompactFormatter;
    use crate::serialize::{Mask, MaskedSerializer};
    use super::*;

    #[test]
    fn happy_path_serialize_deserialize() {
        let mut map: HashMap<&str, HashMap<&str, &str>> = HashMap::new();

        let mut m1 : HashMap<&str, &str> = HashMap::new();
        m1.insert("a", "1");
        m1.insert("b", "2");

        let mut m2 : HashMap<&str, &str> = HashMap::new();
        m1.insert("c", "3");
        m1.insert("d", "4");

        map.insert("m1", m1);
        map.insert("m2", m2);

        let string = serde_json::to_string_pretty(&map).unwrap();

        let string2 = serde_json::to_string_pretty(&(3, 2)).unwrap();

        let mut test = serde_json::Serializer::new(Vec::with_capacity(128));

        test.serialize_bool(true).unwrap();
        test.serialize_bool(true).unwrap();

        example(&mut test);



        //let x = test.serialize_bool(true);

        //let x2 = test.serialize_bool(false);


        let x = 4+4;
        // JsonMasker::from(r#"{ "asdf": "adsf" }"#);

        // let json = serde_json::to_string(&key).unwrap();

        // let parsed_key: LatchableKey = serde_json::from_str(json.as_str()).unwrap();

        // assert_eq!(key, parsed_key);
    }
}
