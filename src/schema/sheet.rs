
use std::fmt;

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

impl fmt::Display for Sheet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {

        write!(f, "{} :: {} :: {} :: {}", self.sheet_name, self.material_master, self.heat_number, self.po_number)
    }
}