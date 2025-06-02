#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Style {
    pub w: Option<u16>,
    pub h: Option<u16>,
    pub p: Option<(u16, u16, u16, u16)>,
    pub h_align: Option<HAlign>,
    pub v_align: Option<VAlign>,
}

impl Default for Style {
    fn default() -> Self {
        Self {
            w: Some(0),
            h: Some(0),
            p: Some((0, 0, 0, 0)),
            h_align: Some(HAlign::Start),
            v_align: Some(VAlign::Start),
        }
    }
}

#[derive(Clone, Debug)]
pub enum HAlign {
    Start,
    Center,
    End,
}

impl Default for HAlign {
    fn default() -> Self {
        HAlign::Start
    }
}

#[derive(Clone, Debug)]
pub enum VAlign {
    Start,
    Center,
    End,
}

impl Default for VAlign {
    fn default() -> Self {
        VAlign::Start
    }
}
