
use std::time::{UNIX_EPOCH, SystemTime};


pub fn time_millis() -> u64 {
    let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
    since_epoch.as_secs() * 1000 + since_epoch.subsec_nanos() as u64 / 1_000_000
}
