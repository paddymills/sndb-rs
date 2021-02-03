
use std::fmt;
use chrono::NaiveDateTime;

#[derive(Debug, sqlx::FromRow)]
pub struct Operator {
    #[sqlx(rename = "CompletedDateTime")]
    timestamp: String,

    #[sqlx(rename = "OperatorName")]
    pub name: String,
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iso8601_fmt = "%F %T";
        let datetime = NaiveDateTime::parse_from_str(&self.timestamp, iso8601_fmt).unwrap();
        let timestamp = datetime.format("%D %R");

        write!(f, "[{}] {}", timestamp, self.name)
    }
}