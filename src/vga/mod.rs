use core::fmt;
use char::ScreenChar;
use color::{Color, ColorCode};
use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;

pub mod char;
pub mod color;

const BUFFER_HEIGHT: u8 = 25;
const BUFFER_WIDTH: u8 = 80;

#[repr(transparent)]
pub struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH as usize]; BUFFER_HEIGHT as usize],
}

pub struct Writer {
    column_pos: u8,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    #![allow(dead_code)]
    pub fn new(color: ColorCode) -> Writer {
        Writer {
            column_pos: 0,
            color_code: color,
            buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            _ => {
                if self.column_pos >= BUFFER_WIDTH {
                    self.new_line();
                }

                self.buffer.chars[BUFFER_HEIGHT as usize - 1][self.column_pos as usize].write(
                    ScreenChar {
                        char: byte,
                        color: self.color_code,
                    }
                );
                self.column_pos += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let char = self.buffer.chars[row as usize][col as usize].read();
                self.buffer.chars[row as usize - 1][col as usize].write(char);
            }
        }
        self.clear_row(BUFFER_HEIGHT-1);
        self.column_pos = 0;
    }

    fn clear_row(&mut self, row: u8) {
        let blank = ScreenChar{
            char: b' ',
            color: self.color_code
        };
        for col in 0 .. BUFFER_WIDTH {
            self.buffer.chars[row as usize][col as usize].write(blank)
        }


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

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_pos: 0,
        color_code: ColorCode::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
    });
}
