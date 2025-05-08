use char::ScreenChar;
use color::ColorCode;

pub mod color;
pub mod char;

const BUFFER_HEIGHT: u8 = 25;
const BUFFER_WIDTH: u8 = 80;

#[repr(transparent)]
pub struct buffer {
    chars: [[ScreenChar; BUFFER_WIDTH as usize]; BUFFER_HEIGHT as usize]
}

pub struct writer {
    column_pos: u8,
    color_code: ColorCode,
    buffer: &'static mut buffer
}

impl writer {

    pub fn write_byte(&mut self, byte: u8) {
        match &[byte;1] {
            b"\n" => self.new_line(),
            _ => {
                if self.column_pos >=BUFFER_WIDTH {
                    self.new_line();
                }


                self.buffer.chars[BUFFER_HEIGHT as usize - 1][self.column_pos as usize] = ScreenChar {
                    char: byte,
                    color: self.color_code
                };
                self.column_pos +=1;
            }
        }
    }

    fn new_line(&mut self){
        todo!()
    }


}
