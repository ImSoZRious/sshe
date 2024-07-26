use std::collections::HashMap;

pub struct Config {
    pub host: String,
    pub columns: HashMap<Key, String>,
}

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub enum Key {
    Hostname,
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
    Key::Hostname,
    Key::User,
    Key::IdentityFile,
    Key::IdentitiesOnly,
    Key::LogLevel,
    Key::Port,
    Key::UserKnownHostsFile,
    Key::PasswordAuthentication,
    Key::StrictHostKeyChecking,
];

impl Config {
    pub fn mock() -> Vec<Self> {
        use Key::*;
        vec![
            Self {
                host: "aaa".to_owned(),
                columns: [
                    (Hostname, "127.0.0.1".to_owned()),
                    (User, "John".to_owned()),
                ]
                .into_iter()
                .collect(),
            },
            Self {
                host: "bbb".to_owned(),
                columns: [(Hostname, "127.0.1.1".to_owned())].into_iter().collect(),
            },
            Self {
                host: "ccc".to_owned(),
                columns: [(User, "JJJ".to_owned())].into_iter().collect(),
            },
            Self {
                host: "dddd".to_owned(),
                columns: [
                    (User, "JJJ".to_owned()),
                    (IdentityFile, "~/.ssh/id_rsa".to_owned()),
                ]
                .into_iter()
                .collect(),
            },
        ]
    }
}

impl Key {
    pub fn str(&self) -> &'static str {
        use Key::*;
        match self {
            Hostname => "Hostname",
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
            "Hostname" => Ok(Hostname),
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
