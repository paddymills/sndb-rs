
use sqlx::mssql::MssqlConnectOptions;
use sqlx::{ConnectOptions};

use std::io::{self, Write};
use std::fs;

// required for `try_next`
use futures::TryStreamExt;

mod schema;

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {

    let mut conn = MssqlConnectOptions::new()
        .host("HIIWINBL18")
        .username("SNUser")
        .password("BestNest1445")
        .connect()
        .await?;
        
    let query = get_query_from_file("sql/program_status.sql")?;

    while let Some(input) = get_user_input() {
    
        let mut rows = sqlx::query_as::<_, schema::Status>(query.as_str())
            .bind(input)
            .fetch(&mut conn);
    
        while let Some(row) = rows.try_next().await? {
            println!("{:?}", row);
        }
    }

    Ok(())
}

fn get_query_from_file(file_name: &str) -> Result<String, io::Error> {
    let sql_query = fs::read_to_string(file_name)
        .expect("Could not read query file");

    Ok(sql_query)
}

fn get_user_input<>() -> Option<String> {
    let mut input = String::new();
    
    print!("Program: ");
    io::stdout().flush().unwrap();

    io::stdin().read_line(&mut input).unwrap();

    match input.trim() {
        "" => None, // ends input loop
        x => Some(String::from(x))
    }
}
