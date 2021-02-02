
use std::io::{Stdout, stdout};

use crossterm::{ExecutableCommand, Result};
use crossterm::terminal::{Clear, ClearType};
use crossterm::cursor::{MoveUp};

use prettytable::{Table, format, Row};

pub struct ResultPrinter {
    pub table: Table,
    pub term: Stdout,
    pub printed_rows: u16
}

impl<'a> ResultPrinter {
    pub fn new(column_headers: Vec<&'a str>) -> Self {
        let mut instance = Self {
            table: Table::new(),
            term: stdout(),
            printed_rows: 0u16,
        };
        
        instance.table.set_format(*format::consts::FORMAT_NO_BORDER_LINE_SEPARATOR);
        instance.table.set_titles(Row::from(column_headers));

        instance
    }

    pub fn print_row(&mut self, items: Vec<&'a str>) -> Result<()> {
        self.table.add_row(Row::from(items));

        self.print_table()?;
        
        Ok(())
    }

    pub fn print_table(&mut self) -> Result<()> {
        // clear table if printed
        if self.printed_rows > 0 {
            self.term.execute(MoveUp(self.printed_rows))?;
            self.term.execute(Clear(ClearType::FromCursorDown))?;
        }

        // print table
        self.table.print(&mut self.term)?;

        // set printed rows
        self.printed_rows = 2 + self.table.len() as u16;
        
        Ok(())
    }
}