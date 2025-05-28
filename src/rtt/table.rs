use crate::rtt::{Cell, CommandsHolder};
use crate::{terminal_clear, terminal_move_to, terminal_print};

#[derive(Debug)]
pub struct Table {
    pub rows: Vec<Vec<Cell>>,
    y: u16,
    commands: CommandsHolder,
}

impl Table {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            y: 0,
            commands: CommandsHolder::new(),
        }
    }

    pub fn row(&mut self, row: Vec<Cell>) -> &mut Self {
        self.rows.push(row);

        self
    }

    fn join_cells(values: Vec<Cell>) -> String {
        if values.is_empty() {
            return "".to_string();
        }

        values
            .iter()
            .map(|u| u.value.clone())
            .collect::<Vec<String>>()
            .join(" | ")
    }

    fn render_start(&mut self) {
        self.commands.push(|| terminal_clear!());
        self.commands.push(move || terminal_move_to!(0, 0));
    }

    fn render_horizontal_line(&mut self) {
        self.commands
            .push(|| terminal_print!("+---------------------------------------+"));
    }

    fn move_to_next_line(&mut self) {
        self.y = self.y + 1;
        let y: u16 = self.y;
        self.commands.push(move || terminal_move_to!(0, y));
    }

    fn render_row(&mut self, row: Vec<Cell>) {
        let row = format!("| {} |", Self::join_cells(row.clone()));
        self.commands.push(move || terminal_print!(row.clone()));
    }

    fn render_end(&self) {
        self.commands.exec_all();
    }

    pub fn render(&mut self) {
        self.render_start();
        let rows = self.rows.clone();
        for row in rows {
            self.render_horizontal_line();
            self.move_to_next_line();
            self.render_row(row);
            self.move_to_next_line();
        }
        self.render_horizontal_line();
        self.move_to_next_line();
        self.render_end();
    }
}
