/* NOTE: These are from the rust source,
 * the print and the _print is modified
 * the _print is my function which writes down the arg given by print!
 */

use x86_64::instructions::interrupts;

use super::WRITER;
use core::fmt;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::macros::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    })
}
