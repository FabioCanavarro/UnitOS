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
            Color::White => 15,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColorCode {
    pub code: u8,
}

impl ColorCode {
    pub fn new(background: Color, foreground: Color) -> ColorCode {
        color_comb(background, foreground)
    }
}

pub fn color_comb(background: Color, foreground: Color) -> ColorCode {
    ColorCode {
        /* NOTE: Shifts the Background to the left so, 0000 0000, first half is backfround, second
         *        half is foreground
         */
        code: u8::from(background) << 4 | u8::from(foreground),
    }
}
