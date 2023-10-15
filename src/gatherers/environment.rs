use crate::types::{fact::{Fact, FactData}};
use std::env;

pub struct EnvironmentData {}

impl EnvironmentData {
}

impl Fact for EnvironmentData {
    fn gather(&self) -> Vec<FactData>{
        let mut vfd: Vec<FactData> = vec![];
        let time_set = self.get_epoch_ms();
        for (key, value) in env::vars() {
            let fd =  FactData{
                name: String::from("env_") + &key,
                value: value,
                time_set: time_set
            };
            vfd.push(fd);
        }
        return vfd;
    }
}