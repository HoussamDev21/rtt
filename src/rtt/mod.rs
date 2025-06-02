#[macro_use]
mod macros;

mod cell;
mod commands_holder;
mod style;
mod table;

pub use cell::Cell;
use commands_holder::CommandsHolder;
pub use style::HAlign;
pub use style::Style;
pub use table::Table;
pub use table::TableConfig;
