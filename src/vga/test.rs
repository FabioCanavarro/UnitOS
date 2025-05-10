use super::*;
use crate::*;

#[test_case]
fn test_println_output() {
    let s = "Testing!!!";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT as usize - 2][i].read();
        assert_eq!(char::from(screen_char.char), c);
    }
}

#[test_case]
fn test_multi_line_print() {
    let s = "Testing";
    let s2 = "Testing2nd!!!";

    println!("{}", s);
    println!("{}", s2);

    // 1st line
    for (i, c) in s.chars().enumerate() {
        let char = WRITER.lock().buffer.chars[BUFFER_HEIGHT as usize - 3][i].read();
        assert_eq!(char::from(char.char), c);
    }

    // 2nd line
    for (i, c) in s2.chars().enumerate() {
        let char = WRITER.lock().buffer.chars[BUFFER_HEIGHT as usize - 2][i].read();
        assert_eq!(char::from(char.char), c);
    }
}
