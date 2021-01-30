
use sqlx::mssql::MssqlConnectOptions;
use sqlx::{ConnectOptions};

use std::io::{self, Write};
use std::fs;

// required for `try_next`
use futures::TryStreamExt;

mod schema;

#[async_std::main]
#[allow(unused_variables, unused_assignments)]
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

        let mut prev_timestamp: Option<String> = None;
        while let Some(row) = rows.try_next().await? {
            // iterates through rows by timestamps in descending order
            // posting (SN100) and updating (SN102) terminate the loop
            // deleting (SN101) will contiue the loop because it can be a re-posting or a delete
            // (re-posting a program sends a SN101 and a SN100, however they do not always go in order)

            match row.trans_type.as_str() {
                "SN100" => { // program posted
                    match prev_timestamp {
                        // previous row was an SN101 and the timestamps do not match (not a repost)
                        Some(ref ts)
                            if row.timestamp().as_str() != ts.as_str()
                                => println!("Program {} was deleted on {}", row.program_name, ts),

                        // else:
                        //  either first row (program last posted)
                        //  or previous line was an SN101 and the timestamps match
                        _ => println!("Program {} is still active", row.program_name),
                    };

                    break;
                },
                "SN101" => { // program deleted
                    prev_timestamp = Some(row.timestamp());
                },
                "SN102" => { // program updated
                    // TODO: fetch sheet and operator data

                    println!("Updated: {:}", row);

                    break;
                },
                _ => unreachable!(),
            };
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
