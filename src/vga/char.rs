use super::color::ColorCode;

#[derive(Debug,Clone, Copy,PartialEq, Eq)]
pub struct ScreenChar {
    char: u8,
    color: ColorCode
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
pub struct buffer {
    chars: [[ScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT]
}
