
use std::fs;

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

        let host = get_user_input("Host: ").unwrap();
        let db = get_user_input("Database: ").unwrap();
        let user = get_user_input("User: ").unwrap();
        let password = get_user_input("Password: ").unwrap();
        
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