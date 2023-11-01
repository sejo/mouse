// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::types::fact::Fact;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env};

pub struct EnvironmentData {}
impl EnvironmentData {}

#[derive(Serialize, Deserialize)]
pub struct EnvironmentValue {
    key: String,
    value: String,
}

impl Fact for EnvironmentData {
    fn gather(&self) -> String {
        let mut outmap: HashMap<String, String> = HashMap::new();
        for (key, value) in env::vars() {
            outmap.insert(key, value);
        }
        serde_json::to_string(&outmap).unwrap()
    }
}
