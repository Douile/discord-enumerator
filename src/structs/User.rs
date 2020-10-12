use serde::{Deserialize, Serialize};

use crate::traits::Snowflake::Snowflake;

pub_fields! {
#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
struct User {
    id: String,
    username: String,
    discriminator: String,
    avatar: Option<String>,
    bot: Option<bool>,
    system: Option<bool>,
    mfa_enabled: Option<bool>,
    locale: Option<String>,
    verified: Option<bool>,
    email: Option<String>,
    flags: Option<u32>,
    premium_type: Option<u32>,
    public_flags: Option<u32>,
}
}

impl User {
    pub fn empty() -> User {
        User {
            id: String::from("NONE"), username: String::from("NONE"), discriminator: String::from("NONE"),
            avatar: None, bot: None, system: None, mfa_enabled: None, locale: None, verified: None,
            email: None, flags: None, premium_type: None, public_flags: None
         }
    }
}

impl Snowflake for User {
    fn snowflake(&self) -> u64 {
        return self.id.parse::<u64>().unwrap_or(0);
    }
}
