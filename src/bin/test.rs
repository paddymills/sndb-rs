
use sndb::printer::ResultPrinter;

use std::{thread, time};

fn main() {

    let mut table = ResultPrinter::new(vec!["First", "Description", "Third"]);

    table.print_row(vec!["a", "b", "c"]).unwrap();

    thread::sleep(time::Duration::from_secs(1));

    table.print_row(vec!["should widen", "narrower"]).unwrap();
}