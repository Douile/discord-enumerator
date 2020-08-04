use std::collections::HashMap;

use crate::structs::Channel::Channel;

pub struct ChannelTree {
    pub channel: Option<Channel>,
    pub children: HashMap<String, Channel>
}

impl ChannelTree {
    pub fn new(channel: Option<Channel>) -> ChannelTree {
        ChannelTree { channel: channel, children: HashMap::<String, Channel>::new() }
    }
}
