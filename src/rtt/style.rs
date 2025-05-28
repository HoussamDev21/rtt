#[derive(Clone, Debug)]
pub struct Style {
    w: Option<u16>,
    h: Option<u16>,
    p: Option<(u16, u16, u16, u16)>,
}
