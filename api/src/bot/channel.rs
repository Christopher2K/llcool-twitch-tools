use dashmap::DashMap;

pub type ChannelRegistry = DashMap<ChannelName, ChannelHandler>;

pub type ChannelName = String;

pub struct ChannelHandler {

}


