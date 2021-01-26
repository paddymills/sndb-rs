extern crate odbc;
// Use this crate and set environment variable RUST_LOG=odbc to see ODBC warnings
extern crate env_logger;
use odbc::safe::AutocommitOff;
use odbc::DiagnosticRecord;
use odbc::ResultSetState::{Data, NoData};
use odbc::{Connection, Environment, Statement};

use std::env;
use std::result::Result;

fn main() {
    env_logger::init();

    let env = Environment::new().map_err(|e| e.unwrap()).unwrap();

    let cs = build_conn_str(&"HIIWINBL5", &"SNDBaseDev").unwrap();

    let mut conn = env
        .connect_with_connection_string(cs.as_str())
        .unwrap()
        .disable_autocommit()
        .unwrap();

    match execute_statement(&conn) {
        Ok(()) => println!("Success"),
        Err(diag) => println!("Error: {:?}", diag),
    }

    let _ = conn.rollback();
}

fn build_conn_str(server: &str, db: &str) -> Result<String, env::VarError> {
    let user = env::var("SNDB_USER").unwrap();
    let pass = env::var("SNDB_PWD").unwrap();

    Ok(format!(
        "DRIVER={{ODBC Driver 17 for SQL Server}};SERVER={};UID={};PWD={};DATABASE={};",
        server, user, pass, db
    ))
}

fn execute_statement<'env>(conn: &Connection<'env, AutocommitOff>) -> Result<(), DiagnosticRecord> {
    let sql_text = "SELECT SheetName, HeatNumber FROM SNDBaseDev.dbo.Stock WHERE SheetName LIKE ?";

    let stmt = Statement::with_parent(conn)?
        .prepare(&sql_text)?
        .bind_parameter(1, &"S0018%")?;

    match stmt.execute()? {
        Data(mut stmt) => {
            let cols = stmt.num_result_cols()?;
            while let Some(mut cursor) = stmt.fetch()? {
                for i in 1..(cols + 1) {
                    match cursor.get_data::<&str>(i as u16)? {
                        Some(val) => print!(" {}", val),
                        None => print!(" NULL"),
                    }
                }
                println!("");
            }
        }
        NoData(_) => println!("Query executed, no data returned"),
    }

    Ok(())
}
