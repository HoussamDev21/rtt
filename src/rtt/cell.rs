use crate::rtt::Style;

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
        self.style = Some(Style {
            w: Some(w),
            ..Style::default()
        });

        self.clone()
    }

    #[allow(dead_code)]
    pub fn height(&mut self, h: u16) -> Self {
        self.style = Some(Style {
            h: Some(h),
            ..Style::default()
        });

        self.clone()
    }

    #[allow(dead_code)]
    pub fn size(&mut self, w: u16, h: u16) -> Self {
        self.style = Some(Style {
            w: Some(w),
            h: Some(h),
            ..Style::default()
        });

        self.clone()
    }
}
