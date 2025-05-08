#![no_std]
#![no_main]

use core::{isize, panic::PanicInfo};

use vga::{color::{color_comb, Color, ColorCode}, Writer};
mod vga;

static HELLO: &[u8] = b"Hello World";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    
/*
    * NOTE: From my understanding
    * the index starts from 0 so 
    * the 0xb8000 should be the first u8 of Hello
    * 0 * 2 + 1 = 1, to put the color code infront
    * then continue on from there
    
    for (i,&byte) in HELLO.iter().enumerate(){
        let step: isize = i as isize * 2;
        unsafe {
            *vga_buffer.offset(step) = byte;
            *vga_buffer.offset(step + 1) = 0xb;

        };
    }
*/


    let mut writer = Writer::new(ColorCode{
        code: color_comb(Color::Black, Color::Red).code
    });

    writer.write_string("HELLOOOO");

    loop {}
}

#[panic_handler]
fn panic (_info : &PanicInfo) -> ! {
    loop {}
}
