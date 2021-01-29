
#[derive(Debug, sqlx::FromRow)]
#[sqlx(rename_all = "PascalCase")]
pub struct Status {
    #[sqlx(rename = "ArcDateTime")]
    timestamp: String,
    
    program_name: String,
    sheet_name: String,
    trans_type: String,
}

#[derive(Debug, sqlx::FromRow)]
#[sqlx(rename_all = "PascalCase")]
pub struct Sheet {
    sheet_name: String,
    heat_number: String,

    #[sqlx(rename = "PrimeCode")]
    material_master: String,

    #[sqlx(rename = "BinNumber")]
    po_number: String,
}

#[derive(Debug, sqlx::FromRow)]
#[sqlx(rename_all = "PascalCase")]
pub struct Operator {
    #[sqlx(rename = "CompletedDateTime")]
    timestamp: String,

    #[sqlx(rename = "OperatorName")]
    name: String,
}
