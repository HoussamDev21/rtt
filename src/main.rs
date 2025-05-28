mod rtt;

use rtt::{Cell, Table};

pub fn main() {
    let mut table = Table::new();
    table.header(vec![
        Cell::new("col 1"),
        Cell::new("col 2"),
        Cell::new("col 3"),
        Cell::new("col 4"),
    ]);
    table.row(vec![
        Cell::new("row 1 col 1"),
        Cell::new("row 1 col 2"),
        Cell::new("row 1 col 3"),
        Cell::new("row 1 col 4"),
    ]);
    table.row(vec![
        Cell::new("row 2 col 1"),
        Cell::new("row 2 col 2"),
        Cell::new("row 2 col 3"),
        Cell::new("row 2 col 4"),
    ]);
    table.row(vec![
        Cell::new("row 3 col 1"),
        Cell::new("row 3 col 2"),
        Cell::new("row 3 col 3"),
        Cell::new("row 3 col 4"),
    ]);
    table.row(vec![
        Cell::new("row 4 col 1"),
        Cell::new("row 4 col 2"),
        Cell::new("row 4 col 3"),
        Cell::new("row 4 col 4"),
    ]);

    table.render();
}
