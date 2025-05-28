use crate::rtt::Style;

#[derive(Clone, Debug)]
pub struct Cell {
    pub value: String,
    pub style: Option<Style>,
}

impl Cell {
    pub fn new(value: &str) -> Self {
        Self {
            value: value.to_string(),
            style: None,
        }
    }
}
