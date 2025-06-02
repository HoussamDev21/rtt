use crate::rtt::{HAlign, Style};

#[derive(Clone, Debug, Default)]
pub struct Cell {
    pub value: Option<String>,
    pub style: Option<Style>,
}

impl Cell {
    pub fn value(&mut self, value: &str) -> Self {
        self.value = Some(String::from(value));

        self.clone()
    }

    pub fn style(&mut self, style: Style) -> Self {
        self.style = Some(style);

        self.clone()
    }

    pub fn width(&mut self, w: u16) -> Self {
        let mut style = self.style.clone().unwrap_or_default();
        style.w = Some(w);
        self.style = Some(style);

        self.clone()
    }

    #[allow(dead_code)]
    pub fn height(&mut self, h: u16) -> Self {
        let mut style = self.style.clone().unwrap_or_default();
        style.h = Some(h);
        self.style = Some(style);

        self.clone()
    }

    #[allow(dead_code)]
    pub fn size(&mut self, w: u16, h: u16) -> Self {
        self.width(w).height(h);

        self.clone()
    }

    pub fn h_align(&mut self, h_align: HAlign) -> Self {
        let mut style = self.style.clone().unwrap_or_default();
        style.h_align = Some(h_align);
        self.style = Some(style);

        self.clone()
    }
}
