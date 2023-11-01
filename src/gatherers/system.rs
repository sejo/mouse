// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use sysinfo::{System, SystemExt};

use crate::types::fact::Fact;

pub struct SystemData {}
impl SystemData {}

impl Fact for SystemData {
    fn gather(&self) -> String {
        if System::IS_SUPPORTED {
            let mut sys = System::new();
            // we only need certain values.
            sys.refresh_cpu();
            sys.refresh_disks();
            sys.refresh_disks_list();
            sys.refresh_memory();
            sys.refresh_system();
            sys.refresh_components();
            sys.refresh_components_list();
            sys.refresh_users_list();
            serde_json::to_string(&sys).unwrap()
        } else {
            "{\"error\": \"Not Supported\"}".to_string()
        }
    }
}
