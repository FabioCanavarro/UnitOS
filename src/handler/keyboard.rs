use core::char;

use lazy_static::lazy_static;
use pc_keyboard::{layouts::Us104Key, DecodedKey, HandleControl, KeyCode, Keyboard, ScancodeSet1};
use spin::Mutex;

use crate::println;

lazy_static!(
    static ref KEYBOARD: Mutex<Keyboard<Us104Key,ScancodeSet1>> = {
            Mutex::new(
                Keyboard::new(ScancodeSet1::new(),Us104Key, HandleControl::Ignore
            )
        )
    };
);

pub fn process_key(byte: u8) -> Result<Option<char>,pc_keyboard::Error> {
    let mut board = KEYBOARD.lock();
    let x = board.add_byte(byte)?;
    match &x {
        Some(_) => (),
        None => return Ok(None)
    }
    
    let decoded = board.process_keyevent(x.unwrap());

    match decoded {
        Some(e) => (),
        None => return Ok(None)
        
    }

    match decoded.unwrap() {
        DecodedKey::Unicode(x) => {
            Ok(Some(x))
        },
        DecodedKey::RawKey(x) => {
            Ok(Some('C'))
        }
    }
}


