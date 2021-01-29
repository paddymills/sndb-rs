
use sqlx::mssql::MssqlConnectOptions;
use sqlx::{ConnectOptions};

use std::io::{self, Write};
use std::fs;

// required for `try_next`
use futures::TryStreamExt;

mod schema;

#[async_std::main]
#[allow(unused_variables)]
async fn main() -> Result<(), sqlx::Error> {
    // create connection
    let mut conn = MssqlConnectOptions::new()
        .host("HIIWINBL18")
        .username("SNUser")
        .password("BestNest1445")
        .connect()
        .await?;
        
    // preload queries
    let get_status = get_query_from_file("sql/program_status.sql")?;
    let get_sheet = get_query_from_file("sql/sheet.sql")?;
    let get_operator = get_query_from_file("sql/operator.sql")?;


    // while let Some(input) = get_user_input() {
    {
        let input = "46064";
        let mut rows = sqlx::query_as::<_, schema::Status>(&get_status)
            .bind(input)
            .fetch(&mut conn);

        while let Some(row) = rows.try_next().await? {
            // iterates through rows by timestamps in descending order
            // posting (SN100) and updating (SN102) terminate the loop
            // deleting (SN101) will contiue the loop because it can be a re-posting or a delete
            // (re-posting a program sends a SN101 and a SN100, however they do not always go in order)

            // match row.trans_type {
            //     "SN100" => { // program posted
            //         println!("Program {} is still active", row.program_name);

            //         break;
            //     },
            //     "SN101" => { // program deleted
            //         println!("Got program deletion.");
            //     },
            //     "SN102" => { // program updated
            //         println!("Updated: {:}", row);

            //         break;
            //     },
            //     _ => unreachable!(),
            // };

            println!("{:}", row);
        }
    }

    Ok(())
}

fn get_query_from_file(file_name: &str) -> Result<String, io::Error> {
    let sql_query = fs::read_to_string(file_name)
        .expect("Could not read query file");

    Ok(sql_query)
}

#[allow(dead_code)]
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
