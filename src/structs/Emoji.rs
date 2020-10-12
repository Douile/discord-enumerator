use serde::{Deserialize, Serialize};

use crate::structs::User::User;

use crate::traits::Snowflake::Snowflake;

pub_fields! {
#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
struct Emoji {
    id: Option<String>,
    name: Option<String>,
    roles: Option<Vec<String>>,
    user: Option<User>,
    require_colons: Option<bool>,
    managed: Option<bool>,
    animated: Option<bool>,
    available: Option<bool>,
}
}

impl Emoji {
    pub fn name_safe(&self, default: String) -> String {
        match &self.name {
            Some(name) => {
                return name.clone();
            }
            None => {
                return default;
            }
        }
    }

    pub fn id_safe(&self, default: String) -> String {
        match &self.id {
            Some(id) => {
                return id.clone();
            }
            None => {
                return default;
            }
        }
    }
}

impl Snowflake for Emoji {
    fn snowflake(&self) -> u64 {
        match &self.id {
            Some(id) => {
                return id.parse::<u64>().unwrap_or(0);
            }
            None => {
                return 0;
            }
        }
    }
}
