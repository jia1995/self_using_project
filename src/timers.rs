//! This lib is aiming at generate current time as i64.
use std::time::SystemTime;

/// Get current nanos time.
/// ```
/// let cur = current_time_as_naons();
/// ```
pub fn current_time_as_nanos() -> i64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(t) => t.as_nanos() as i64,
        Err(e) => panic!("Time is illegal!! Error msg {}.", e),
    }
}

/// Get current micros time.
/// ```
/// let cur = current_time_as_micros();
/// ```
pub fn current_time_as_micros() -> i64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(t) => t.as_micros() as i64,
        Err(e) => panic!("Time is illegal!! Error msg {}.", e),
    }
}

/// Get current millis time.
/// ```
/// let cur = current_time_as_millis();
/// ```
pub fn current_time_as_millis() -> i64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(t) => t.as_millis() as i64,
        Err(e) => panic!("Time is illegal!! Error msg {}.", e),
    }
}

/// Get current secs time.
/// ```
/// let cur = current_time_as_secs();
/// ```
pub fn current_time_as_secs() -> i64 {
    match SystemTime::now().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(t) => t.as_secs() as i64,
        Err(e) => panic!("Time is illegal!! Error msg {}.", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        use std::thread::sleep;
        use std::time::Duration;
        let result = current_time_as_millis();
        dbg!("{:?}", result);
        sleep(Duration::new(1,0));
        let result = current_time_as_secs();
        dbg!("{:?}", result);
        sleep(Duration::new(1,0));
        let result = current_time_as_micros();
        dbg!("{:?}", result);
        sleep(Duration::new(1,0));
    }
}