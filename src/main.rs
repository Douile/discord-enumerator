extern crate reqwest;
extern crate tokio;

use std::collections::HashMap;
use std::env;

#[macro_use]
mod macros;

mod structs;
use crate::structs::Channel::Channel;
use crate::structs::ChannelTree::ChannelTree;
use crate::structs::DiscordError::DiscordError;
use crate::structs::Guild::Guild;
use crate::structs::GuildMember::GuildMember;
use crate::structs::Role::Role;

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
        // println!("{:#?}", guild);
        println!("{} [{}] {}", guild.name, guild.id, guild.region);

        let mut roles = HashMap::<String, Role>::new();
        for role in &guild.roles {
            roles.insert(role.clone().id, role.clone());
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
        log_roles(&guild.roles).await?;

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

    println!("{} channel groups", channels.len());
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

async fn log_roles(roles: &Vec<Role>) -> Result<(), Box<dyn std::error::Error>> {
    let mut roles = roles.clone();
    println!("{} roles", roles.len());
    roles.sort_by(|a, b| a.position.cmp(&b.position));
    for role in roles {
        println!("{}[{}:{}] Color:{:#X} Hoist:{} Managed:{} Mentionable:{} Perms:{}", role.name, role.position, role.id, role.color, role.hoist, role.managed, role.mentionable, role.permissions_new);
    }

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
