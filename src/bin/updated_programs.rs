
use sqlx::mssql::{MssqlConnectOptions, MssqlPool};

use prettytable::{Row, Cell};

// required for `try_next`
use futures::TryStreamExt;

use sndb::*;
use sndb::printer::ResultPrinter;

#[async_std::main]
#[allow(unused_variables, unused_assignments)]
async fn main() -> Result<(), sqlx::Error> {
    // read config
    let cfg = config::DbConfig::from("db.toml");

    // create connection
    let db_cfg = MssqlConnectOptions::new()
        .host(&cfg.host)
        .database(&cfg.db)
        .username(&cfg.user)
        .password(&cfg.password);
    
    let pool = MssqlPool::connect_lazy_with(db_cfg);

    let header = vec!["Program", "Status", "Timestamp", "SAP MM", "Heat Number", "PO Number", "Operator"];
    let mut term = ResultPrinter::new(header);


    let mut program = String::from("00000");
    while let Some(input) = get_user_input("\nProgram: ") {
        term.printed_rows += 2;
        
        match 5 - input.len() {
            x if x < 5 => program.replace_range(x.., &input),
            _ => program = String::from(input),
        }
        
        let mut rows = sqlx::query_as::<_, schema::Status>(schema::queries::PROGRAM_STATUS)
            .bind(&program)
            .fetch(&pool);
        
        let mut prev_timestamp: Option<String> = None;
        let mut table_row = Row::new(Vec::new());
        while let Some(row) = rows.try_next().await? {
            // iterates through rows by timestamps in descending order
            // posting (SN100) and updating (SN102) terminate the loop
            // deleting (SN101) will contiue the loop because it can be a re-posting or a delete
            // (re-posting a program sends a SN101 and a SN100, however they do not always go in order)
            
            if table_row.is_empty() {
                table_row.add_cell(Cell::new(&row.program_name));
            }

            match row.trans_type.as_str() {
                "SN100" => { // program posted
                    match prev_timestamp {
                        // previous row was an SN101
                        //  and the timestamps do not match (not a repost)
                        Some(ref ts)
                            if row.timestamp().as_str() != ts.as_str()
                                => {
                                    table_row.add_cell(Cell::new("Deleted"));
                                    table_row.add_cell(Cell::new(ts));
                                },

                        // else:
                        //  either
                        //      first row (program last posted)
                        //  or
                        //      previous line was an SN101 and the timestamps match
                        _ => {
                            table_row.add_cell(Cell::new("Active"));
                            table_row.add_cell(Cell::new(&row.timestamp()));
                        },
                    };

                    break;
                },
                "SN101" => { // program deleted
                    match prev_timestamp {
                        Some(ref ts) => {
                            // previous row was another 101
                            table_row.add_cell(Cell::new("Deleted"));
                            table_row.add_cell(Cell::new(ts));

                            break;
                        },
                        None => prev_timestamp = Some(String::from(row.timestamp()))
                    }

                    // next iteration checks if timestamps match
                    //  to determine if this is a repost or delete
                    continue
                },
                "SN102" => { // program updated

                    table_row.add_cell(Cell::new("Updated"));
                    table_row.add_cell(Cell::new(&row.timestamp()));

                    // get sheet data
                    let sheet = sqlx::query_as::<_, schema::Sheet>(schema::queries::SHEET)
                        .bind(&row.program_name)
                        .bind(&row.sheet_name)
                        .fetch_one(&pool)
                        .await?;

                    table_row.add_cell(Cell::new(&sheet.material_master));
                    table_row.add_cell(Cell::new(&sheet.heat_number));
                    table_row.add_cell(Cell::new(&sheet.po_number));
                    
                    // get operator
                    let operator = match sqlx::query_as::<_, schema::Operator>(schema::queries::OPERATOR)
                        .bind(&row.program_name)
                        .bind(&row.sheet_name)
                        .fetch_one(&pool)
                        .await {
                            Ok(result) => result.name,
                            _ => String::from("--"),
                        };
                        
                    table_row.add_cell(Cell::new(&operator));

                    break;
                },
                _ => unreachable!(),
            }
        }

        // input returned no results
        if table_row.is_empty() {
            table_row.add_cell(Cell::new(&program));
            table_row.add_cell(Cell::new("Not found"));         
        }

        // add row to table
        term.table.add_row(table_row);
        
        // print table
        term.print_table().unwrap();
    }

    Ok(())
}
