use std::str::FromStr;

#[derive(Debug)]
pub enum TwitchMessage {
    RplWelcome,
    Ping,
    Unknown(String),
}

impl FromStr for TwitchMessage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items: Vec<&str> = s.split(" ").collect::<_>();

        if let Some(ping) = items.get(0) {
            if ping == &"PING" {
                return Ok(Self::Ping);
            }
        };

        if let Some(code) = items.get(1) {
            let parsed_code = match *code {
                "001" => Self::RplWelcome,
                _ => Self::Unknown(String::from(s)),
            };
            Ok(parsed_code)
        } else {
            Ok(Self::Unknown(String::from(s)))
        }
    }
}

#[derive(Debug)]
pub enum BotMessage {
    JoinChat(String),
    LeaveChat(String),
    Pong,
}

#[derive(Debug)]
pub enum ConnectedChannelsSetMessage {
    Join(String),
    Leave(String),
}
