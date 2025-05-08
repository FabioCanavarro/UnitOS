#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White,
}

impl From<Color> for u8 {
    fn from(value: Color) -> Self {
        match value {
            Color::Black => 0,
            Color::Blue => 1,
            Color::Green => 2,
            Color::Cyan => 3,
            Color::Red => 4,
            Color::Magenta => 5,
            Color::Brown => 6,
            Color::LightGray => 7,
            Color::DarkGray => 8,
            Color::LightBlue => 9,
            Color::LightGreen => 10,
            Color::LightCyan => 11,
            Color::LightRed => 12,
            Color::Pink => 13,
            Color::Yellow => 14,
            Color::White => 15
        }
    }
}

struct ColorCode{
    code: u8
}

fn color_comb (background: Color, foreground: Color) -> ColorCode {
    background << 4 | foreground
}
