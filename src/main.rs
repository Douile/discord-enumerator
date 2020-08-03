extern crate reqwest;
extern crate tokio;

use std::collections::HashMap;
use std::env;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    approximate_presence_count: Option<u32>
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
enum ChannelType {
    GUILD_TEXT = 0,
    DM = 1,
    GUILD_VOICE = 2,
    GROUP_DM = 3,
    GUILD_CATEGORY = 4,
    GUILD_NEWS = 5,
    GUILD_STORE = 6
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Channel {
    id: String,
    r#type: ChannelType,
    guild_id: Option<String>,
    position: Option<u32>,
    permission_overwrites: Option<Vec<Overwrite>>,
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
    last_pin_timestamp: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Overwrite {
    id: String,
    r#type: String,
    allow: u32,
    allow_new: String,
    deny: u32,
    deny_new: String
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    public_flags: Option<u32>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GuildMember {
    user: Option<User>,
    nick: Option<String>,
    roles: Vec<String>,
    joined_at: String,
    premium_since: String,
    deaf: bool,
    mute: bool
}


#[derive(Serialize, Deserialize, Debug, Clone)]
struct Role {
    id: String,
    name: String,
    color: u32,
    hoist: bool,
    position: u8,
    permissions: u32,
    permissions_new: String,
    managed: bool,
    mentionable: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Emoji {
    id: Option<String>,
    name: Option<String>,
    roles: Option<Vec<String>>,
    user: Option<User>,
    require_colons: Option<bool>,
    managed: Option<bool>,
    animated: Option<bool>,
    available: Option<bool>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
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
    suppress: bool
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Presence {
    user: User,
    roles: Vec<String>,
    game: Option<Activity>,
    guild_id: String,
    status: String,
    activities: Vec<Activity>,
    client_status: ClientStatus,
    premium_since: Option<String>,
    nick: Option<String>
}

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone)]
#[repr(u8)]
enum ActivityType {
    Game = 0,
    Streaming = 1,
    Listening = 2,
    Custom = 4
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct Activity {
    name: String,
    r#type: ActivityType,
    url: Option<String>,
    created_at: u64,
    timestamps: Option<ActivityTimestamps>,
    application_id: Option<String>,
    details: Option<String>,
    state: Option<String>,
    emoji: Option<Emoji>,
    party: Option<ActivityParty>,
    assets: Option<ActivityAssets>,
    secrets: Option<ActivitySecrets>,
    instance: Option<bool>,
    flags: Option<u8>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ActivityTimestamps {
    start: Option<u64>,
    end: Option<u64>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ActivityParty {
    id: Option<String>,
    size: Option<[u8; 2]>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ActivityAssets {
    large_image: Option<String>,
    large_text: Option<String>,
    small_image: Option<String>,
    small_text: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ActivitySecrets {
    join: Option<String>,
    spectate: Option<String>,
    r#match: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct ClientStatus {
    desktop: Option<String>,
    mobile: Option<String>,
    web: Option<String>
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct DiscordError {
    code: u32,
    message: String
}

impl std::fmt::Display for DiscordError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DiscordError {}: {}", self.code, self.message)
    }
}

impl std::error::Error for DiscordError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut i: u8 = 0;
    let mut token : String = String::from("");
    let mut id : String = String::from("");
    for argument in env::args() {
        if i == 1 {
            token = argument;
        } else if i == 2 {
            id = argument;
        } else if i > 2 {
            break;
        }
        i += 1;
    }

    println!("Token:\"{}\" Guild:\"{}\"", token, id);

    if i < 3 {
        println!("Not enough args");
    } else {
        let guild = fetch_guild(id.as_str().clone().to_string(), token.as_str().clone().to_string()).await?;
        println!("{:#?}", guild);
        println!("{} [{}] {}", guild.name, guild.id, guild.region);

        let mut roles = HashMap::<String, Role>::new();
        for role in guild.roles {
            roles.insert(role.clone().id, role);
        }

        match guild.channels {
            Some(channels) => {
                log_channels(channels, roles).await?;
            }
            None => {
                log_channels(fetch_guild_channels(id, token).await?, roles).await?;
            }
        }

        // let channels = fetch_guild_channels(id, token).await;
        // println!("{:#?}", channels);
    }
    Ok(())
}

async fn log_channels(channels: Vec<Channel>, roles: HashMap<String, Role>) -> Result<(), Box<dyn std::error::Error>> {
    println!("{} channels", channels.len());
    for channel in channels {
        println!("#{} [{}] - {:#?}", channel.name, channel.id, channel.topic);
        match channel.permission_overwrites {
            Some(permissions) => {
                for permission in permissions {
                    match permission.r#type.as_str() {
                        "member" => {
                            println!("\tUser[{}] Allow: {} Deny: {}", permission.id, permission.allow_new, permission.deny_new);
                        },
                        "role" => {
                            match roles.get(&permission.id) {
                                Some(role) => println!("\tRole[{}] Allow: {} Deny: {}", role.name, permission.allow_new, permission.deny_new),
                                None => println!("\tRole[{}] Allow: {} Deny: {}", permission.id, permission.allow_new, permission.deny_new)
                            }
                        },
                        _ => println!("\tUnknown[{}] Allow: {} Deny: {}", permission.id, permission.allow_new, permission.deny_new)
                    }
                }
            }
            None => {}
        }
    }
    Ok(())
}

async fn fetch_guild(guild_id: String, token: String) -> Result<Guild, Box<dyn std::error::Error>> {
    let response = reqwest::Client::new()
        .get(format!("https://discord.com/api/v6/guilds/{}", guild_id).as_str())
        .header("Authorization", token)
        .send()
        .await?;

    if response.status().is_success() {
        let guild = response.json::<Guild>().await?;
        Ok(guild)
    } else {
        let err = response.json::<DiscordError>().await?;
        Err(Box::new(err))
    }
}

async fn fetch_guild_channels(guild_id: String, token: String) -> Result<Vec<Channel>, Box<dyn std::error::Error>> {
    let response = reqwest::Client::new()
        .get(format!("https://discord.com/api/v6/guilds/{}/channels", guild_id).as_str())
        .header("Authorization", token)
        .send()
        .await?;

    if response.status().is_success() {
        let channels = response.json::<Vec<Channel>>().await?;
        Ok(channels)
    } else {
        let err = response.json::<DiscordError>().await?;
        Err(Box::new(err))
    }
}

async fn fetch_guild_member(guild_id: String, user_id: String, token: String) -> Result<GuildMember, Box<dyn std::error::Error>> {
    let response = reqwest::Client::new()
        .get(format!("https://discord.com/api/v6/guilds/{}/members/{}", guild_id, user_id).as_str())
        .header("Authorization", token)
        .send()
        .await?;

    if response.status().is_success() {
        let user = response.json::<GuildMember>().await?;
        Ok(user)
    } else {
        let err = response.json::<DiscordError>().await?;
        Err(Box::new(err))
    }
}
