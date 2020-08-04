#[macro_use]

use serde::{Deserialize, Serialize};

use crate::structs::User::User;

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
