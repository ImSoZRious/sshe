use std::collections::HashMap;

pub struct Config {
    pub host: String,
    pub columns: HashMap<Key, String>,
}

#[derive(Clone, PartialEq, Eq, Hash, Copy)]
pub enum Key {
    Hostname = 0,
    User,
}

pub const ALL_KEYS: [Key; 2] = [Key::Hostname, Key::User];

impl Config {
    pub fn mock() -> Vec<Self> {
        vec![
            Self {
                host: "aaa".to_owned(),
                columns: [
                    (Key::Hostname, "127.0.0.1".to_owned()),
                    (Key::User, "John".to_owned()),
                ]
                .into_iter()
                .collect(),
            },
            Self {
                host: "bbb".to_owned(),
                columns: [(Key::Hostname, "127.0.1.1".to_owned())]
                    .into_iter()
                    .collect(),
            },
            Self {
                host: "ccc".to_owned(),
                columns: [(Key::User, "JJJ".to_owned())].into_iter().collect(),
            },
        ]
    }
}

impl Key {
    pub fn str(&self) -> &'static str {
        match self {
            Key::Hostname => "Hostname",
            Key::User => "User",
        }
    }
}
