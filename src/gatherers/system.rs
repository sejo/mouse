// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use std::{fs, io::ErrorKind, str, usize};

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
            "bios_date": get_string("/sys/devices/virtual/dmi/id/bios_date"),
            "bios_vendor": get_string("/sys/devices/virtual/dmi/id/bios_vendor"),
            "bios_version": get_string("/sys/devices/virtual/dmi/id/bios_version"),
            "board_asset_tag": get_string("/sys/devices/virtual/dmi/id/board_asset_tag"),
            "board_name": get_string("/sys/devices/virtual/dmi/id/board_name"),
            "board_serial": get_string("/sys/devices/virtual/dmi/id/board_serial"),
            "board_vendor": get_string("/sys/devices/virtual/dmi/id/board_vendor"),
            "board_version": get_string("/sys/devices/virtual/dmi/id/board_version"),
            "chassis_asset_tag": get_string("/sys/devices/virtual/dmi/id/chassis_asset_tag"),
            "chassis_serial": get_string("/sys/devices/virtual/dmi/id/chassis_vendor"),
            "chassis_vendor": get_string("/sys/devices/virtual/dmi/id/chassis_vendor"),
            "chassis_version": get_string("/sys/devices/virtual/dmi/id/chassis_version"),
            "product_name": get_string("/sys/devices/virtual/dmi/id/product_name"),
            "product_serial": get_string("/sys/devices/virtual/dmi/id/product_serial"),
            "product_uuid": get_string("/sys/devices/virtual/dmi/id/product_uuid"),
            "product_version": get_string("/sys/devices/virtual/dmi/id/product_version"),
            "system_vendor": get_string("/sys/devices/virtual/dmi/id/sys_vendor"),
        })
        .to_string()
    }
}

fn get_string(path: &str) -> String {
    match fs::read(path) {
        Ok(x) => String::from_utf8(x).unwrap().trim().to_string(),
        Err(y) => match y.kind() {
            ErrorKind::PermissionDenied => "This data needs sudo or root permissions".to_string(),
            other_error => format!("Could not retrieve {other_error}"),
        },
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
