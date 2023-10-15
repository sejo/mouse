use std::time::{UNIX_EPOCH, SystemTime};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct FactData {
    pub name: String,
    pub value: String,
    pub time_set: u128, 
}

pub trait Fact : Send + Sync {
    fn gather(&self) -> Vec<FactData>;

    fn get_epoch_ms(&self) -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
    }
}