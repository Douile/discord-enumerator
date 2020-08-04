#[macro_use]

use serde::{Deserialize, Serialize};

use crate::structs::User::User;

pub_fields! {
#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
struct GuildMember {
    user: Option<User>,
    nick: Option<String>,
    roles: Vec<String>,
    joined_at: String,
    premium_since: Option<String>,
    deaf: bool,
    mute: bool,
}
}

impl GuildMember {
    pub fn get_user(&self) -> User {
        match &self.user {
            Some(user) => user.clone(),
            None => User::empty()
        }
    }
}
