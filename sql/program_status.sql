USE SNDBase91;

SELECT
    ArcDateTime,
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