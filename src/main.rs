mod rtt;
use rtt::{Cell, Style, Table};

pub fn main() {
    let mut table = Table::new();

    let mut cell = Cell::default();
    cell.style(Style {
        w: Some(0),
        p: Some((0, 1, 0, 1)),
        ..Default::default()
    });

    table.row(vec![
        cell.value("name").width(30),
        cell.value("color").width(15),
        cell.value("price").width(15),
        cell.value("quantity").width(15),
    ]);
    table.row(vec![
        cell.value("car toy"),
        cell.value("red"),
        cell.value("$12"),
        cell.value("100"),
    ]);
    table.row(vec![
        cell.value("xbox controller"),
        cell.value("white"),
        cell.value("$70"),
        cell.value("50"),
    ]);
    table.row(vec![
        cell.value("fancy keyboard"),
        cell.value("black"),
        cell.value("$49.99"),
        cell.value("33"),
    ]);

    table.render();
}
