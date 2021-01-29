USE SNDBase91;

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