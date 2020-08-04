#[macro_use]

use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::structs::PermissionOverwrite::PermissionOverwrite;
use crate::structs::User::User;

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum ChannelType {
    GUILD_TEXT = 0,
    DM = 1,
    GUILD_VOICE = 2,
    GROUP_DM = 3,
    GUILD_CATEGORY = 4,
    GUILD_NEWS = 5,
    GUILD_STORE = 6
}

pub_fields! {
#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
struct Channel {
    id: String,
    r#type: ChannelType,
    guild_id: Option<String>,
    position: Option<u32>,
    permission_overwrites: Option<Vec<PermissionOverwrite>>,
    name: String,
    topic: Option<String>,
    nsfw: Option<bool>,
    last_message_id: Option<String>,
    bitrate: Option<u32>,
    user_limit: Option<u32>,
    rate_limit_per_user: Option<u32>,
    recipients: Option<Vec<User>>,
    icon: Option<String>,
    owner_id: Option<String>,
    application_id: Option<String>,
    parent_id: Option<String>,
    last_pin_timestamp: Option<String>,
}
}
