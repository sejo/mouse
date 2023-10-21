use std::process::Command;

use crate::types::fact::{Fact, FactData};
use serde::{Deserialize, Serialize};

pub struct SystemData {}

impl SystemData {}



#[derive(Serialize, Deserialize)]
struct UnameData{
    kernel_name: String,
    node_name: String,
    kernel_release: String,
    kernel_version: String,
    machine: String,
    processor: String,
    hardware_platform: String,
    operating_system: String,
}

#[derive(Serialize, Deserialize)]
struct IpAddrInfo {
    family: String,
    local: String,
    prefixlen: i32,
    broadcast: Option<String>,
    scope: String,
    dynamic: Option<bool>,
    noprefixroute: Option<bool>,
    label: Option<String>,
    valid_life_time: i64,
    preferred_life_time: i64,
}

#[derive(Serialize, Deserialize)]
struct IpAddr {
    ifindex: i32,
    ifname: String,
    flags: Vec<String>,
    mtu: i32,
    qdisc: String,
    operstate: String,
    group: String,
    txqlen: i32,
    link_type: String,
    address: Option<String>,
    broadcast: Option<String>,
    addr_info: Vec<IpAddrInfo>,
}

impl Fact for SystemData {
    fn gather(&self) -> Vec<FactData> {
        let mut vfd: Vec<FactData> = vec![];
        let time_set = self.get_epoch_ms();
        let output = Command::new("ip")
            .arg("-j")
            .arg("addr")
            .output()
            .expect("Failed to execute ip");
        let stdout = String::from_utf8(output.stdout).unwrap();
        let ips: Vec<IpAddr> = serde_json::from_str(stdout.as_str()).unwrap();
        for ip in ips {
            if ip.ifname != "lo" {
                for addr_info in ip.addr_info {
                    let fd = FactData {
                        name: format!("ip_addr_{}_{}_addr", ip.ifname, addr_info.family),
                        value: addr_info.local,
                        time_set: time_set,
                    };
                    vfd.push(fd);
                    vfd.push(FactData {
                        name: format!("ip_addr_{}_{}_prefixlen", ip.ifname, addr_info.family),
                        value: addr_info.prefixlen.to_string(),
                        time_set: time_set,
                    });
                    if addr_info.label.is_some() {
                        vfd.push(FactData {
                            name: format!("ip_addr_{}_{}_label", ip.ifname, addr_info.family),
                            value: addr_info.label.unwrap(),
                            time_set: time_set,
                        });
                    }
                    if addr_info.broadcast.is_some() {
                        vfd.push(FactData {
                            name: format!("ip_addr_{}_{}_broadcast", ip.ifname, addr_info.family),
                            value: addr_info.broadcast.unwrap(),
                            time_set: time_set,
                        });
                    }
                    if ip.address.is_some() {
                        vfd.push(FactData {
                            name: format!("ip_addr_{}_macaddr", ip.ifname),
                            value: ip.address.clone().unwrap(),
                            time_set: time_set,
                        })
                    }
                }
            }
        }
        return vfd;
    }
}
