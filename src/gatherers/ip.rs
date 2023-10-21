use crate::{types::fact::{Fact, FactData}, util::command::get_result_as_string};
use serde::{Deserialize, Serialize};

pub struct IPData {}

impl IPData {}

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

impl Fact for IPData {
    fn gather(&self) -> Vec<FactData> {
        let mut vfd: Vec<FactData> = vec![];
        let time_set = self.get_epoch_ms();
        let ips: Vec<IpAddr> = serde_json::from_str(get_result_as_string("ip", vec!["-j", "addr"]).as_str()).unwrap();
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

#[derive(Serialize, Deserialize)]
struct IpRoute {
    dst: String,
    gateway: Option<String>,
    dev: String,
    protocol: Option<String>,
    metric: Option<i32>,
    flags: Vec<String>,
    scope: String,
    prefsrc: String, 

}

impl Fact for IpRoute {
    fn gather(&self) -> Vec<FactData> {
        let mut vfd: Vec<FactData> = vec![];
        let time_set = self.get_epoch_ms();
        
        let routes: Vec<IpRoute> = serde_json::from_str(get_result_as_string("ip", vec!["-j", "route"]).as_str()).unwrap();

        return vfd;
    }
}