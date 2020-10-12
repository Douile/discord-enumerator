use chrono::prelude::DateTime;
use chrono::Utc;
use std::time::{UNIX_EPOCH, Duration};

pub trait Snowflake {
    fn snowflake(&self) -> u64;

    fn created(&self) -> u64 {
        return (self.snowflake() >> 22) + 1420070400000;
    }

    fn created_datetime(&self) -> DateTime<Utc> {
        let created = self.created();
        let d = UNIX_EPOCH + Duration::from_millis(created);
        let datetime = DateTime::<Utc>::from(d);
        return datetime;
    }
}
