
use sqlx::mssql::{MssqlConnectOptions, MssqlPool};

use sndb::*;

#[async_std::main]
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


}
