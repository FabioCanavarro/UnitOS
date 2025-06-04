use core::char;

use lazy_static::lazy_static;
use pc_keyboard::{DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1, layouts::Us104Key};
use spin::Mutex;

use crate::println;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<Us104Key, ScancodeSet1>> = {
        Mutex::new(Keyboard::new(
            ScancodeSet1::new(),
            Us104Key,
            HandleControl::Ignore,
        ))
    };
};

// I AM LOSING MY FUCKING MIND bro
pub fn process_key(byte: u8) -> Result<Option<DecodedKey>, pc_keyboard::Error> {
    let mut board = KEYBOARD.lock();

    // NOTE: Might change later, cuz of inefficiency cuz twice
    let x = board.add_byte(byte)?;
    match &x {
        Some(_) => (),
        None => return Ok(None),
    }

    // Fuck this shit, imma do this only
    Ok(board.process_keyevent(x.unwrap()))
}
