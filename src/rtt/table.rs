use crate::rtt::{Cell, CommandsHolder, HAlign};
use crate::{terminal_clear, terminal_move_to, terminal_print};

#[derive(Debug)]
pub struct Table {
    pub rows: Vec<Vec<Cell>>,
    y: u16,
    commands: CommandsHolder,
    config: Option<TableConfig>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            rows: Vec::new(),
            y: 0,
            commands: CommandsHolder::new(),
            config: Some(TableConfig::default()),
        }
    }

    pub fn set_config(&mut self, config: TableConfig) {
        self.config = Some(config)
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
        let config = self.config.as_ref().cloned().unwrap_or_default();

        let line: Vec<String> = cells_w
            .iter()
            .map(|len| String::from(config.h_line_char.repeat(*len as usize)))
            .collect();

        self.commands.push(move || {
            terminal_print!(format!(
                "{}{}{}",
                config.corners_char,
                line.join(config.corners_char),
                config.corners_char,
            ))
        });
    }

    fn format_cell(&self, (index, cell): (usize, &Cell)) -> String {
        let cells_w = &self.cells_w();
        let style = cell.style.clone().unwrap_or_default();
        let (_, pe, _, ps) = style.p.unwrap_or_default();

        let h_align = style.h_align.unwrap_or_default();
        let cell_value = cell.value.clone().unwrap_or_default();

        let ps = ps as usize;
        let pe = pe as usize;
        let width = (cells_w[index] - ps as u16 - pe as u16) as usize;

        let (pre, val, post) = match h_align {
            HAlign::Start => (
                format!("{:<ps$}", "", ps = ps),
                format!("{:<width$}", cell_value, width = width),
                format!("{:<pe$}", "", pe = pe),
            ),
            HAlign::Center => (
                format!("{:^ps$}", "", ps = ps),
                format!("{:^width$}", cell_value, width = width),
                format!("{:^pe$}", "", pe = pe),
            ),
            HAlign::End => (
                format!("{:>ps$}", "", ps = ps),
                format!("{:>width$}", cell_value, width = width),
                format!("{:>pe$}", "", pe = pe),
            ),
        };

        format!("{pre}{val}{post}")
    }

    fn join_cells(&self, values: Vec<Cell>) -> String {
        let config = self.config.as_ref().cloned().unwrap_or_default();

        values
            .iter()
            .enumerate()
            .map(|it| self.format_cell(it))
            .collect::<Vec<String>>()
            .join(config.v_line_char)
    }

    fn move_to_next_line(&mut self) {
        self.y = self.y + 1;
        let y: u16 = self.y;
        self.commands.push(move || terminal_move_to!(0, y));
    }

    fn format_row(&self, row: Vec<Cell>) -> String {
        let config = self.config.as_ref().cloned().unwrap_or_default();

        format!(
            "{}{}{}",
            config.v_line_char,
            self.join_cells(row.clone()),
            config.v_line_char
        )
    }

    fn format_empty_row(&self, row: Vec<Cell>) -> String {
        let config = self.config.as_ref().cloned().unwrap_or_default();

        format!(
            "{}{}{}",
            config.v_line_char,
            self.join_cells(
                row.clone()
                    .iter()
                    .map(|cell| cell.clone().value(""))
                    .collect()
            ),
            config.v_line_char
        )
    }

    fn render_row(&mut self, row: Vec<Cell>, height: u16) {
        let min_height = if height == 0 { 1 } else { height };
        self.move_to_next_line();
        for i in 0..min_height {
            let row_formatted = if i == 0 {
                self.format_row(row.clone())
            } else {
                self.format_empty_row(row.clone())
            };
            self.commands
                .push(move || terminal_print!(row_formatted.clone()));
            self.move_to_next_line();
        }
    }

    fn render_end(&self) {
        self.commands.exec_all();
    }

    pub fn render(&mut self) {
        let rows_h = self.rows_h();

        self.render_start();
        let rows = self.rows.clone();
        for (index, row) in rows.into_iter().enumerate() {
            self.render_horizontal_line();
            self.render_row(row, rows_h[index]);
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

    fn rows_h(&self) -> Vec<u16> {
        let rows_h = self
            .rows
            .iter()
            .map(|row| {
                row.iter()
                    .map(|cell| cell.clone().style.unwrap_or_default().h.unwrap_or_default())
                    .max()
                    .unwrap_or_default()
            })
            .collect();

        rows_h
    }
}

#[derive(Debug, Clone)]
pub struct TableConfig {
    pub corners_char: &'static str,
    pub h_line_char: &'static str,
    pub v_line_char: &'static str,
}

impl Default for TableConfig {
    fn default() -> Self {
        Self {
            corners_char: "+",
            h_line_char: "-",
            v_line_char: "|",
        }
    }
}
