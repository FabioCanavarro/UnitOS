#![no_std]
#![no_main]

use core::panic::PanicInfo;
use vga::{
    Writer,
    color::{Color, ColorCode, color_comb},
};
mod vga;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
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

    use core::fmt::Write;
    let mut writer = Writer::new(ColorCode {
        code: color_comb(Color::Black, Color::Red).code,
    });

    writeln!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();


    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
