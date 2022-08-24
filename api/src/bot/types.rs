use std::str::FromStr;

#[derive(Debug)]
pub enum TwitchMessage {
    RplWelcome,
    Unknown(String),
}

impl FromStr for TwitchMessage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let items = s.split(" ").collect::<Vec<&str>>();

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
}
