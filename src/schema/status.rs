
use std::fmt;
use chrono::NaiveDateTime;

#[derive(Debug, Default, sqlx::FromRow)]
#[sqlx(rename_all = "PascalCase")]
pub struct Status {
    arc_date_time: String,
    program_name: String,
    sheet_name: String,
    trans_type: String,

    #[sqlx(default)]
    timestamp: Option<String>,
}

// impl Default for Status {
//     fn default() -> Self {

//     }
// }

impl fmt::Display for Status {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let iso8601_fmt = "%F %T";
        let datetime = NaiveDateTime::parse_from_str(&self.arc_date_time, iso8601_fmt).unwrap();
        let timestamp = datetime.format("%D %R");

        write!(f, "[{}] ({}) {} :: {}", timestamp, self.trans_type, self.program_name, self.sheet_name)
    }
}