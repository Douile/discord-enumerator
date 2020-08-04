use serde::{Deserialize, Serialize};

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
    position: u8,
    permissions: u32,
    permissions_new: String,
    managed: bool,
    mentionable: bool,
}
}
