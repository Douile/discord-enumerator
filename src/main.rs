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
#[allow(non_camel_case_types)]
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
impl User {
    fn empty() -> User {
        User {
            id: String::from("NONE"), username: String::from("NONE"), discriminator: String::from("NONE"),
            avatar: None, bot: None, system: None, mfa_enabled: None, locale: None, verified: None,
            email: None, flags: None, premium_type: None, public_flags: None
         }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
struct GuildMember {
    user: Option<User>,
    nick: Option<String>,
    roles: Vec<String>,
    joined_at: String,
    premium_since: Option<String>,
    deaf: bool,
    mute: bool
}

impl GuildMember {
    fn get_user(&self) -> User {
        match &self.user {
            Some(user) => user.clone(),
            None => User::empty()
        }
    }
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

struct ChannelTree {
    channel: Option<Channel>,
    children: HashMap<String, Channel>
}

impl ChannelTree {
    fn new(channel: Option<Channel>) -> ChannelTree {
        ChannelTree { channel: channel, children: HashMap::<String, Channel>::new() }
    }
}

static USER_AGENT: &'static str = "Reqwest/0.10 DiscordEnumerator/0.1";
static NO_TOPIC: &'static str = "No topic";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut i: u8 = 0;
    let mut token = String::from("");
    let mut id = String::from("");
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
        let guild = fetch_guild(&id, &token).await?;
        println!("{:#?}", guild);
        println!("{} [{}] {}", guild.name, guild.id, guild.region);

        let mut roles = HashMap::<String, Role>::new();
        for role in guild.roles {
            roles.insert(role.clone().id, role);
        }

        let channel_tree = match guild.channels {
            Some(channels) => {
                //log_channels(channels, roles, &token).await?;
                construct_channel_tree(channels).await?
            }
            None => {
                //log_channels(fetch_guild_channels(&id, &token).await?, roles, &token).await?;
                construct_channel_tree(fetch_guild_channels(&id, &token).await?).await?
            }
        };

        log_channels(channel_tree, &roles, &token).await?;

        // let channels = fetch_guild_channels(id, token).await;
        // println!("{:#?}", channels);
    }

    Ok(())
}

async fn construct_channel_tree(channels: Vec<Channel>) -> Result<HashMap<String, ChannelTree>, Box<dyn std::error::Error>> {
    let mut root = HashMap::<String, ChannelTree>::new();
    for channel in channels {
        match channel.clone().parent_id {
            Some(parent_id) => {
                match root.get_mut(&parent_id) {
                    Some(parent) => {
                        parent.children.insert(channel.clone().id, channel);
                    },
                    None => {
                        let mut parent = ChannelTree::new(None);
                        parent.children.insert(channel.clone().id, channel);
                        root.insert(parent_id, parent);
                    }
                }
            }
            None => {
                match root.get_mut(&channel.id) {
                    Some(node) => {
                        node.channel.replace(channel);
                    }
                    None => {
                        root.insert(channel.clone().id, ChannelTree::new(Some(channel)));
                    }
                }
            }
        }
    }

    Ok(root)
}

async fn log_channels(channels: HashMap<String, ChannelTree>, roles: &HashMap<String, Role>, token: &String) -> Result<(), Box<dyn std::error::Error>> {
    let mut members = HashMap::<String, GuildMember>::new();

    println!("{} channels", channels.len());
    for channel_tree in channels.values() {
        match &channel_tree.channel {
            Some(channel) => {
                log_channel(0, &channel, roles, &mut members, token).await?;
            }
            None => {
                println!("Dead parent");
            }
        }
        for channel_child in channel_tree.children.values() {
            log_channel(1, &channel_child, roles, &mut members, token).await?;
        }

    }
    Ok(())
}

async fn log_channel(indentation: u32, channel: &Channel, roles: &HashMap<String, Role>, members: &mut HashMap<String, GuildMember>, token: &String) -> Result<(), Box<dyn std::error::Error>> {
    println!("{}#{} [{}] - {:#?}", calc_indentation(indentation), channel.name, channel.id, channel.topic.as_ref().unwrap_or(&NO_TOPIC.to_string()));
    let inner_indent = calc_indentation(indentation+1);
    match &channel.permission_overwrites {
        Some(permissions) => {
            let guild_id: String = match &channel.guild_id {
                Some(guild_id) => guild_id.to_string(),
                None => String::from("None")
            };
            for permission in permissions {
                match permission.r#type.as_str() {
                    "member" => {
                        match members.get(&permission.id) {
                            Some(member) => println!("{}User[{}.{}] Allow: {} Deny: {}", &inner_indent, permission.clone().id, member.get_user().username, permission.allow_new, permission.deny_new),
                            None => {
                                let result = fetch_guild_member(&guild_id, &permission.id, &token).await;
                                match result {
                                    Ok(member) => {
                                        members.insert(permission.clone().id, member.clone());
                                        println!("{}User[{}.{}] Allow: {} Deny: {}", &inner_indent, permission.clone().id, member.get_user().username, permission.allow_new, permission.deny_new);
                                    }
                                    Err(e) => {
                                        println!("Error fetching user: {:#?}", e);
                                        println!("{}User[{}] Allow: {} Deny: {}", &inner_indent, permission.clone().id, permission.allow_new, permission.deny_new)
                                    }
                                }
                            }
                        }

                    },
                    "role" => {
                        match roles.get(&permission.id) {
                            Some(role) => println!("{}Role[{}.{}] Allow: {} Deny: {}", &inner_indent, role.id, role.name, permission.allow_new, permission.deny_new),
                            None => println!("{}Role[{}] Allow: {} Deny: {}", &inner_indent, permission.id, permission.allow_new, permission.deny_new)
                        }
                    },
                    _ => println!("{}Unknown[{}] Allow: {} Deny: {}", &inner_indent, permission.id, permission.allow_new, permission.deny_new)
                }
            }
        }
        None => {}
    };

    Ok(())
}

async fn fetch_guild(guild_id: &String, token: &String) -> Result<Guild, Box<dyn std::error::Error>> {
    let response = reqwest::Client::new()
        .get(format!("https://discord.com/api/v6/guilds/{}", guild_id).as_str())
        .header("Authorization", token)
        .header("User-Agent", USER_AGENT)
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

async fn fetch_guild_channels(guild_id: &String, token: &String) -> Result<Vec<Channel>, Box<dyn std::error::Error>> {
    let response = reqwest::Client::new()
        .get(format!("https://discord.com/api/v6/guilds/{}/channels", guild_id).as_str())
        .header("Authorization", token)
        .header("User-Agent", USER_AGENT)
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

async fn fetch_guild_member(guild_id: &String, user_id: &String, token: &String) -> Result<GuildMember, Box<dyn std::error::Error>> {
    let response = reqwest::Client::new()
        .get(format!("https://discord.com/api/v6/guilds/{}/members/{}", guild_id, user_id).as_str())
        .header("Authorization", token)
        .header("User-Agent", USER_AGENT)
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

fn calc_indentation(i: u32) -> String {
    if i == 0 {
        String::new()
    } else if i == 1 {
        String::from("├ ")
    } else {
        let mut res = String::new();
        for _ in 1..i {
            res.push_str("  ");
        }
        res.push_str("├ ");
        res
    }
}
