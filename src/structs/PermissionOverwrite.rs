#[macro_use]

use serde::{Deserialize, Serialize};

pub_fields! {
#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
struct PermissionOverwrite {
    id: String,
    r#type: String,
    allow: u32,
    allow_new: String,
    deny: u32,
    deny_new: String,
}
}
