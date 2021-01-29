USE OYSProgramUpdate;

SELECT
    CompletedDateTime,
    OperatorName
FROM
    CompletedProgram
WHERE
    ProgramName = @P1
AND
    SheetName = @P2