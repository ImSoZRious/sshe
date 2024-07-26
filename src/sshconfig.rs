use std::collections::HashMap;

pub struct Config {
    pub host: String,
    pub columns: HashMap<Key, String>,
}

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub enum Key {
    HostName,
    User,
    IdentityFile,
    IdentitiesOnly,
    LogLevel,
    Port,
    UserKnownHostsFile,
    PasswordAuthentication,
    StrictHostKeyChecking,
}

pub const ALL_KEYS: [Key; 9] = [
    Key::HostName,
    Key::User,
    Key::IdentityFile,
    Key::IdentitiesOnly,
    Key::LogLevel,
    Key::Port,
    Key::UserKnownHostsFile,
    Key::PasswordAuthentication,
    Key::StrictHostKeyChecking,
];

impl Key {
    pub fn str(&self) -> &'static str {
        use Key::*;
        match self {
            HostName => "HostName",
            User => "User",
            IdentityFile => "IdentityFile",
            IdentitiesOnly => "IdentitiesOnly",
            LogLevel => "LogLevel",
            Port => "Port",
            UserKnownHostsFile => "UserKnownHostsFile",
            PasswordAuthentication => "PasswordAuthentication",
            StrictHostKeyChecking => "StrictHostKeyChecking",
        }
    }
}

impl TryFrom<&str> for Key {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Key::*;
        match value {
            "HostName" => Ok(HostName),
            "User" => Ok(User),
            "IdentityFile" => Ok(IdentityFile),
            "IdentitiesOnly" => Ok(IdentitiesOnly),
            "LogLevel" => Ok(LogLevel),
            "Port" => Ok(Port),
            "UserKnownHostsFile" => Ok(UserKnownHostsFile),
            "PasswordAuthentication" => Ok(PasswordAuthentication),
            "StrictHostKeyChecking" => Ok(StrictHostKeyChecking),
            _ => Err(()),
        }
    }
}
