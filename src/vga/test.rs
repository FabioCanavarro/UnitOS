use crate::*;
use super::*;

#[test_case]
fn test_println_output() {
    let s = "Testing!!!";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT as usize - 2][i].read();
        assert_eq!(char::from(screen_char.char), c);
    }
}
