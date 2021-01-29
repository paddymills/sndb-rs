USE OYSProgramUpdate;

SELECT
    CONVERT(nvarchar, CompletedDateTime, 120) AS CompletedDateTime,
    OperatorName
FROM
    CompletedProgram
WHERE
    ProgramName = @P1
AND
    SheetName = @P2