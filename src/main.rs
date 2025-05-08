#![no_std]
#![no_main]

use core::panic::PanicInfo;

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


    println!("Hi there");
    println!("How's life lol");
    println!("Not sure bout yours, but i love mine lol");
    println!("Love my family so much");
    println!("Love programming so much too");
    println!("Exercising here and there");
    panic!("Something wong with u lol");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}",info);
    loop {}
}
