use std::time::{SystemTime, UNIX_EPOCH};

pub trait Fact: Send + Sync {
    fn gather(&self) -> String;

    fn get_epoch_ms(&self) -> u128 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis()
    }
}
