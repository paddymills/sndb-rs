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