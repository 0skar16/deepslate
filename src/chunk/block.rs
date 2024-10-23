use std::collections::HashMap;

use bitcode::{Decode, Encode};

use crate::{PropName, PropValue};

#[derive(Debug, Encode, Decode, Clone, PartialEq)]
pub struct BlockState {
    pub block: String,
    pub properties: HashMap<PropName, PropValue>,
}

impl From<String> for BlockState {
    fn from(value: String) -> Self {
        let (block, s_state) = value.split_once("|").expect("Couldn't split blockstateid");
        let mut properties = HashMap::new();
        for prop in s_state.to_string().split(",") {
            let prop = prop.to_string();
            if prop.len() == 0 {
                continue;
            }
            let (name, value) = prop.split_once("=").expect("Couldn't split prop");
            properties.insert(
                PropName::from_str(name),
                PropValue::from_str(value),
            );
        }
        Self { block: block.to_string(), properties }
    }
}

impl BlockState {
    pub fn new(block: impl ToString) -> Self {
        Self {
            block: block.to_string(),
            properties: HashMap::new(),
        }
    }
    pub fn set(&mut self, name: PropName, val: PropValue) -> Option<PropValue> {
        self.properties.insert(name, val)
    }
    pub fn get(&mut self, name: PropName) -> Option<&PropValue> {
        self.properties.get(&name)
    }
    pub fn remove(&mut self, name: PropName) -> Option<PropValue> {
        self.properties.remove(&name)
    }
}