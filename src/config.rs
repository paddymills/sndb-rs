
use std::{fs, fmt};

use serde::{Serialize, Deserialize};
use toml;

use crate::get_user_input;

#[derive(Debug, Serialize, Deserialize)]
pub struct DbConfig {
    pub host: String,
    pub db: String,
    pub user: String,
    pub password: String,
}

impl DbConfig {
    pub fn new() -> Self {

        let env_or_user_input = |var, prompt| -> String {
            match std::env::var(var) {
                Ok(val) => val,
                Err(_) => get_user_input(format!("SigmaNest Database {}: ", prompt).as_str()).unwrap(),
            }
        };
        
        let host = env_or_user_input("SNDB_HOST", "Host");
        let db = env_or_user_input("SNDB_DB", "Database");
        let user = env_or_user_input("SNDB_USER", "User");
        let password = env_or_user_input("SNDB_PWD", "Password");

        Self { host, db, user, password }
    }

    pub fn from(file_name: &str) -> Self {
        // read file
        match fs::read_to_string(file_name) {
            Ok(text) => toml::from_str::<Self>(&text).unwrap(),
            _ => {
                let cfg = Self::new();

                // write to disk
                match toml::to_string(&cfg) {
                    Ok(as_toml) => match fs::write(file_name, as_toml) {
                        Ok(_) => (),
                        Err(_) => println!("Failed to write config"),
                    },
                    Err(_) => println!("Failed to write serialize config"),
                }

                cfg
            },
        }
    }
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            host: String::new(),
            db: String::new(),
            user: String::new(),
            password: String::new()
        }
    }
}

impl fmt::Display for DbConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} :: {} :: {} :: {}", self.host, self.db, self.user, self.password)
    }
}