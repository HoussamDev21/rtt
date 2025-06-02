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

    fn render_start(&mut self) {
        self.commands.push(|| terminal_clear!());
        self.commands.push(move || terminal_move_to!(0, 0));
    }

    fn render_horizontal_line(&mut self) {
        let cells_w = &self.cells_w();

        let line: Vec<String> = cells_w
            .iter()
            .map(|len| String::from("-".repeat(*len as usize)))
            .collect();

        self.commands
            .push(move || terminal_print!(format!("+{}+", line.join("+"))));
    }

    fn join_cells(&self, values: Vec<Cell>) -> String {
        let cells_w = &self.cells_w();

        values
            .iter()
            .enumerate()
            .map(|(i, c)| {
                format!(
                    "{:<width$}",
                    c.value.clone().unwrap_or_default(),
                    width = cells_w[i] as usize
                )
            })
            .collect::<Vec<String>>()
            .join("|")
    }

    fn move_to_next_line(&mut self) {
        self.y = self.y + 1;
        let y: u16 = self.y;
        self.commands.push(move || terminal_move_to!(0, y));
    }

    fn render_row(&mut self, row: Vec<Cell>) {
        let row = format!("|{}|", self.join_cells(row.clone()));
        self.commands.push(move || terminal_print!(row.clone()));
    }

    fn render_end(&self) {
        self.commands.exec_all();
    }

    pub fn render(&mut self) {
        self.render_start();
        let rows = self.rows.clone();
        self.cells_w();
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

    fn cells_w(&self) -> Vec<u16> {
        // populate cells_w using the first row
        let first_row = self.rows.get(0).unwrap();
        let mut cells_w: Vec<u16> = first_row
            .into_iter()
            .map(|cell| cell.clone().style.unwrap_or_default().w.unwrap_or_default())
            .collect();

        for row in self.rows.iter() {
            for (ci, _) in row.iter().enumerate() {
                let w = self
                    .rows
                    .iter()
                    .map(|row| {
                        row[ci]
                            .clone()
                            .style
                            .unwrap_or_default()
                            .w
                            .unwrap_or_default()
                    })
                    .max(); // use the biggest w value
                cells_w[ci] = w.unwrap_or_default();
            }
        }

        cells_w
    }
}
