#[macro_use]

use serde::{Deserialize, Serialize};

use crate::structs::GuildMember::GuildMember;

pub_fields! {
#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
struct VoiceState {
    guild_id: Option<String>,
    channel_id: Option<String>,
    user_id: String,
    member: Option<GuildMember>,
    session_id: String,
    deaf: bool,
    mute: bool,
    self_deaf: bool,
    self_mute: bool,
    self_stream: Option<bool>,
    self_video: bool,
    suppress: bool,
}
}
