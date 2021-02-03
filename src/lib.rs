
pub mod schema;
pub mod printer;
pub mod config;

use std::io::{self, Write};
use std::fs;

#[allow(dead_code)]
pub fn get_query_from_file(file_name: &str) -> Result<String, io::Error> {
    let sql_query = fs::read_to_string(file_name)
        .expect("Could not read query file");

    Ok(sql_query)
}

#[allow(dead_code)]
pub fn get_user_input(prompt: &str) -> Option<String> {
    let mut input = String::new();
    
    print!("{}", prompt);
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "" => None, // ends input loop
        x => Some(x.to_string())
    }
}
