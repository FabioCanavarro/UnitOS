use spin::Mutex;
use pic8259::ChainedPics;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: Mutex<ChainedPics> = 
    Mutex::new(
        unsafe {
            ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET)
        }
);

#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard = PIC_1_OFFSET + 1,
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }
}

