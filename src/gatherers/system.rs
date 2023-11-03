// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{fs, str, usize};

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

pub struct DMIData {}
impl DMIData {}

impl Fact for DMIData {
    fn gather(&self) -> String {
        json!({
            "form_factor": get_form_factor("/sys/devices/virtual/dmi/id/chassis_type"),
        })
        .to_string()
    }
}

fn get_form_factor(path: &str) -> &str {
    // Get form factor info
    // as learned from Ansible, we need to have the list in a specific order,
    // see https://www.dmtf.org/sites/default/files/standards/documents/DSP0134_3.2.0.pdf

    let ff: Vec<&str> = vec![
        "Unknown",
        "Other",
        "Unknown",
        "Desktop",
        "Low Profile Desktop",
        "Pizza Box",
        "Mini Tower",
        "Tower",
        "Portable",
        "Laptop",
        "Notebook",
        "Hand Held",
        "Docking Station",
        "All In One",
        "Sub Notebook",
        "Space-saving",
        "Lunch Box",
        "Main Server Chassis",
        "Expansion Chassis",
        "Sub Chassis",
        "Bus Expansion Chassis",
        "Peripheral Chassis",
        "RAID Chassis",
        "Rack Mount Chassis",
        "Sealed-case PC",
        "Multi-system",
        "CompactPCI",
        "AdvancedTCA",
        "Blade",
        "Blade Enclosure",
        "Tablet",
        "Convertible",
        "Detachable",
        "IoT Gateway",
        "Embedded PC",
        "Mini PC",
        "Stick PC",
    ];

    let filecontent: &str = match fs::read(path) {
        Err(_) => ff[0],
        Ok(x) => {
            let ind: i32 = match str::from_utf8(&x) {
                Ok(x) => x.trim().parse::<i32>().unwrap(),
                Err(_) => 0,
            };
            let out: &str = if ff.len() <= ind as usize {
                ff[0]
            } else {
                ff[ind as usize]
            };
            out
        }
    };
    filecontent
}
