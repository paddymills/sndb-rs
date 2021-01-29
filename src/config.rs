
use serde::{Serialize, Deserialize};
use confy;

#[derive(Debug, Serialize, Deserialize)]
struct DbConfig {
    host: String,
    user: String,
    password: String,
}

impl Default for DbConfig {
    fn default() -> Self {
        Self {
            host: String::new(),
            user: String::new(),
            password: String::new()
        }
    }
}