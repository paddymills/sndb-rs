
use std::fmt;
use chrono::NaiveDateTime;

#[derive(Debug, Default, sqlx::FromRow)]
#[sqlx(rename_all = "PascalCase")]
pub struct Status {
    arc_date_time: String,
    pub program_name: String,
    pub sheet_name: String,
    pub trans_type: String,

    #[sqlx(default)]
    timestamp: Option<String>,
}

#[allow(dead_code)]
impl Status {
    pub fn timestamp(&self) -> String {
        match &self.timestamp {
            Some(ts) => ts.to_string(),
            None => {
                let iso8601_fmt = "%F %T";
                let datetime = NaiveDateTime::parse_from_str(&self.arc_date_time, iso8601_fmt).unwrap();

                datetime.format("%D %R").to_string()
            }
        }
    }
}

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        
        write!(f, "[{}] ({}) {} :: {}", self.timestamp(), self.trans_type, self.program_name, self.sheet_name)

    }
}