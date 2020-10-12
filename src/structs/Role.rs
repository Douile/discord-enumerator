use serde::{Deserialize, Serialize};

use crate::traits::Snowflake::Snowflake;

pub_fields! {
#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
struct Role {
    id: String,
    name: String,
    color: u32,
    hoist: bool,
    position: u16,
    permissions: u32,
    permissions_new: String,
    managed: bool,
    mentionable: bool,
}
}

impl Snowflake for Role {
    fn snowflake(&self) -> u64 {
        return self.id.parse::<u64>().unwrap_or(0);
    }
}
