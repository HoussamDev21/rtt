mod rtt;
use rtt::{Cell, Style, Table};

use crate::rtt::{HAlign, TableConfig};

pub fn main() {
    let mut table = Table::new();

    table.set_config(TableConfig {
        corners_char: ".",
        h_line_char: "=",
        v_line_char: "|",
    });

    let mut cell = Cell::default();
    cell.style(Style {
        w: Some(0),
        p: Some((0, 1, 0, 1)),
        h_align: Some(HAlign::Start),
        ..Default::default()
    });

    table.row(vec![
        cell.value("name").width(30).h_align(HAlign::Center),
        cell.value("color").width(15).h_align(HAlign::Center),
        cell.value("price").width(15).h_align(HAlign::Center),
        cell.value("quantity").width(15).h_align(HAlign::Center),
    ]);
    table.row(vec![
        cell.value("car toy"),
        cell.value("red"),
        cell.value("$12").h_align(HAlign::End),
        cell.value("100").h_align(HAlign::End),
    ]);
    table.row(vec![
        cell.value("xbox controller"),
        cell.value("white"),
        cell.value("$70").h_align(HAlign::End),
        cell.value("50").h_align(HAlign::End),
    ]);
    table.row(vec![
        cell.value("fancy keyboard"),
        cell.value("black"),
        cell.value("$49.99").h_align(HAlign::End),
        cell.value("33").h_align(HAlign::End),
    ]);

    table.render();
}
