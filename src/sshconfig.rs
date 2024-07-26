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
}

pub const ALL_KEYS: [Key; 3] = [Key::Hostname, Key::User, Key::IdentityFile];

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
        }
    }
}
