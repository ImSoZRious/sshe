use std::collections::HashMap;

macro_rules! count {
    ($key:ident, $($other:ident),*) => {
        1 + count!($($other),*)
    };

    ($key:ident) => {
        1
    }
}

macro_rules! key_literal {
    ($($key:ident),*) => {
        #[derive(Clone, PartialEq, Eq, Hash, Copy)]
        pub enum Key {
            $(
                $key,
            )*
        }

        pub const ALL_KEYS: [Key; count!($($key),*)] = [
            $(
                Key::$key,
            )*
        ];

        
        impl Key {
            pub fn str(&self) -> &'static str {
                use Key::*;
                match self {
                    $(
                        $key => stringify!($key),
                    )*
                }
            }
        }

        impl TryFrom<&str> for Key {
            type Error = ();

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                use Key::*;
                match value {
                    $(
                        stringify!($key) => Ok($key),
                    )*
                    _ => Err(()),
                }
            }
        }
    };
}

key_literal!(
    HostName,
    User,
    IdentityFile,
    IdentitiesOnly,
    LogLevel,
    Port,
    UserKnownHostsFile,
    PasswordAuthentication,
    StrictHostKeyChecking
);

pub struct Config {
    pub host: String,
    pub columns: HashMap<Key, String>,
}
