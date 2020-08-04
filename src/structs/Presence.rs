use serde::{Deserialize, Serialize};

use crate::structs::Activity::Activity;
use crate::structs::ClientStatus::ClientStatus;
use crate::structs::User::User;

pub_fields! {
#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
struct Presence {
    user: User,
    roles: Vec<String>,
    game: Option<Activity>,
    guild_id: String,
    status: String,
    activities: Vec<Activity>,
    client_status: ClientStatus,
    premium_since: Option<String>,
    nick: Option<String>,
}
}
