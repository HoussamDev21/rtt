#[allow(dead_code)]
#[derive(Clone, Debug, Default)]
pub struct Style {
    pub w: Option<u16>,
    pub h: Option<u16>,
    pub p: Option<(u16, u16, u16, u16)>,
}
