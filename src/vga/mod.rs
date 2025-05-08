use char::ScreenChar;
use color::ColorCode;

pub mod color;
pub mod char;

const BUFFER_HEIGHT: u8 = 25;
const BUFFER_WIDTH: u8 = 80;

#[repr(transparent)]
pub struct Buffer {
    chars: [[ScreenChar; BUFFER_WIDTH as usize]; BUFFER_HEIGHT as usize]
}

pub struct Writer {
    column_pos: u8,
    color_code: ColorCode,
    buffer: &'static mut Buffer
}

impl Writer {

    pub fn new(color: ColorCode) -> Writer {
        Writer { 
            column_pos: 0,
            color_code: color,
            buffer: unsafe {
                &mut *(0xb8000 as *mut Buffer)
            }
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
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

    pub fn write_string(&mut self, string: &str) {
        for byte in string.as_bytes() {
            match byte {
            /* NOTE: This code diffrentiates the byte, since the vga writer supports some
            *   additional char that isnt writable on rust utf-8, so we match if it is in the range
            *   of the writeable or is a new line
            *   if not, we print ■■■, just one tho
            */
            0x20..=0x7e | b'\n' => self.write_byte(*byte),
            _ => self.write_byte(0xfe),

        }
        }
        

    }
}
