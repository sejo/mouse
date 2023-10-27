use crate::types::fact::Fact;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::env;

pub struct EnvironmentData {}
impl EnvironmentData {}

#[derive(Serialize, Deserialize)]
pub struct EnvironmentValue {
    key: String,
    value: String,
    time_set: u128,
}

impl Fact for EnvironmentData {
    fn gather(&self) -> String {
        let mut outmap: Vec<Value> = vec![];
        for (key, value) in env::vars() {
            let entry = json!({
                "key": &key,
                "value": value,
            });
            outmap.append(&mut vec![entry]);
        }
        serde_json::to_string(&outmap).unwrap()
    }
}
