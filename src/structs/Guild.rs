#[macro_use]

use serde::{Deserialize, Serialize};

use crate::structs::Channel::Channel;
use crate::structs::Emoji::Emoji;
use crate::structs::GuildMember::GuildMember;
use crate::structs::Presence::Presence;
use crate::structs::Role::Role;
use crate::structs::VoiceState::VoiceState;

pub_fields! {
#[derive(Serialize)]
#[derive(Deserialize)]
#[derive(Debug)]
#[derive(Clone)]
struct Guild {
    id: String,
    name: String,
    icon: Option<String>,
    splash: Option<String>,
    discovery_splash: Option<String>,
    owner: Option<bool>,
    owner_id: Option<String>,
    permissions: Option<u32>,
    permissions_new: Option<String>,
    region: String,
    afk_channel_id: Option<String>,
    afk_timeout: u32,
    embed_enabled: Option<bool>,
    embed_channel_id: Option<String>,
    verification_level: u8,
    default_message_notifications: u8,
    explicit_content_filter: u8,
    roles: Vec<Role>,
    emojis: Vec<Emoji>,
    features: Vec<String>,
    mfa_level: u8,
    application_id: Option<String>,
    widget_enabled: Option<bool>,
    widget_channel_id: Option<String>,
    system_channel_id: Option<String>,
    system_channel_flags: Option<u8>,
    rules_channel_id: Option<String>,
    joined_at: Option<String>,
    large: Option<bool>,
    unavailable: Option<bool>,
    member_count: Option<u32>,
    voice_states: Option<Vec<VoiceState>>,
    members: Option<Vec<GuildMember>>,
    channels: Option<Vec<Channel>>,
    presences: Option<Vec<Presence>>,
    max_presences: Option<u32>,
    max_members: u64,
    vanity_url_code: Option<String>,
    description: Option<String>,
    banner: Option<String>,
    premium_tier: u8,
    premium_subscription_count: u32,
    preferred_locale: String,
    public_updates_channel: Option<String>,
    max_video_channel_users: Option<u32>,
    approximate_member_count: Option<u32>,
    approximate_presence_count: Option<u32>,
}
}
