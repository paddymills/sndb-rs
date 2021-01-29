
use std::fmt;
use chrono::NaiveDateTime;

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
pub struct Operator {
    #[sqlx(rename = "CompletedDateTime")]
    timestamp: String,

    #[sqlx(rename = "OperatorName")]
    name: String,
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iso8601_fmt = "%F %T";
        let datetime = NaiveDateTime::parse_from_str(&self.timestamp, iso8601_fmt).unwrap();
        let timestamp = datetime.format("%D %R");

        write!(f, "[{}] ({}) {} :: {}", timestamp, self.trans_type, self.program_name, self.sheet_name)
    }
}


impl fmt::Display for Sheet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "{} :: {} :: {} :: {}", self.sheet_name, self.material_master, self.heat_number, self.po_number)
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iso8601_fmt = "%F %T";
        let datetime = NaiveDateTime::parse_from_str(&self.timestamp, iso8601_fmt).unwrap();
        let timestamp = datetime.format("%D %R");

        write!(f, "[{}] {}", timestamp, self.name)
    }
}