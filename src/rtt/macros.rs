#[macro_export]
macro_rules! terminal_clear {
    () => {
        crossterm::execute!(
            std::io::stdout(),
            crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
        )
    };
}

#[macro_export]
macro_rules! terminal_move_to {
    ($x:expr, $y:expr) => {
        crossterm::execute!(std::io::stdout(), crossterm::cursor::MoveTo($x, $y))
    };
}

#[macro_export]
macro_rules! terminal_print {
    ($value:expr) => {
        crossterm::execute!(std::io::stdout(), crossterm::style::Print($value))
    };
}
