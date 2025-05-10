use crate::*;

#[test_case]
fn tassert() {
    serial_print!("tassert.... ");
    assert_eq!(1, 1);
    serial_print!("[OK]");
}
