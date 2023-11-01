// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use crate::{types::fact::Fact, util::command::get_result_as_string};
use serde_json::Value;

pub struct IPData {}
impl IPData {}

impl Fact for IPData {
    fn gather(&self) -> String {
        let res = get_result_as_string("ip", vec!["-j", "addr"]);
        let jsonres: Value = serde_json::from_str(res.as_str()).unwrap();
        let out = json!({ "ipaddr": jsonres });
        out.to_string()
    }
}
pub struct IPRouteData {}
impl IPRouteData {}

impl Fact for IPRouteData {
    fn gather(&self) -> String {
        let res = get_result_as_string("ip", vec!["-j", "route"]);
        let jsonres: Value = serde_json::from_str(res.as_str()).unwrap();
        let out = json!({
            "iproute": jsonres
        });
        out.to_string()
    }
}
