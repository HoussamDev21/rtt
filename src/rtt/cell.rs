#[derive(Clone, Debug)]
pub struct Cell {
    pub value: String,
}

impl Cell {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
        }
    }
}
