
use sqlx::mssql::MssqlConnectOptions;
use sqlx::{ConnectOptions};

// required for `try_next`
use futures::TryStreamExt;

#[derive(sqlx::FromRow)]
#[sqlx(rename_all = "PascalCase")]
struct Plate {
    sheet_name: String,
    heat_number: String,
}

#[async_std::main]
async fn main() -> Result<(), sqlx::Error> {
    // let sql_text = "SELECT * FROM Stock WHERE SheetName LIKE ?";

    let mut conn = MssqlConnectOptions::new()
        .host("HIIWINBL5")
        .database("SNDBaseDev")
        .username("SNUser")
        .password("BestNest1445")
        .connect()
        .await?;

    let mut rows = sqlx::query_as::<_, Plate>("
    SELECT * FROM Stock
    WHERE SheetName LIKE @P1
    AND HeatNumber LIKE @P2
    ")
        .bind(&"S0018%")
        .bind(&"D%")
        .fetch(&mut conn);
    

    while let Some(row) = rows.try_next().await? {
        // let sheet: String = row.try_get("SheetName")?;
        // let heat: String = row.try_get("HeatNumber")?;

        println!("{} :: {}", row.sheet_name, row.heat_number);
    }

    Ok(())
}
