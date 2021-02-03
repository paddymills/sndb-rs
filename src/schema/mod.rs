
mod status;
mod sheet;
mod operator;

pub use status::Status;
pub use sheet::Sheet;
pub use operator::Operator;

pub mod queries {
    pub const PROGRAM_STATUS: &str = "
        SELECT
            CONVERT(nvarchar, ArcDateTime, 120) AS ArcDateTime,
            ProgramName,
            SheetName,
            TransType
        FROM
            ProgArchive
        WHERE
            ProgramName = @P1
        ORDER BY
            ArcDateTime
        DESC
    ";

    pub const OPERATOR: &str = "
        SELECT
            CONVERT(nvarchar, CompletedDateTime, 120) AS CompletedDateTime,
            OperatorName
        FROM
            CompletedProgram
        WHERE
            ProgramName = @P1
        AND
            SheetName = @P2
    ";

    pub const SHEET: &str = "
        SELECT
            SheetName,
            PrimeCode,
            HeatNumber,
            BinNumber
        FROM
            StockHistory
        WHERE
            ProgramName = @P1
        AND
            SheetName = @P2
    ";
}