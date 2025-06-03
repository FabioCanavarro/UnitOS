use lazy_static::lazy_static;
use pc_keyboard::{layouts::Us104Key, HandleControl, Keyboard, ScancodeSet2};
use spin::Mutex;

fn before_lazy_static(){
}
lazy_static!(
    static ref KEYBOARD: Mutex<Keyboard<Us104Key,ScancodeSet2>> = {
            Mutex::new(
                Keyboard::new(ScancodeSet2::new(),Us104Key, HandleControl::Ignore
            )
        )
    };
);


