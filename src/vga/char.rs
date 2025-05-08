use super::color::ColorCode;

#[derive(Debug,Clone, Copy,PartialEq, Eq)]
pub struct ScreenChar {
    pub char: u8,
    pub color: ColorCode
}


