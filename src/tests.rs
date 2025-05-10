use crate::*;

pub trait Tests {
    fn run(&self);
}
impl<T> Tests for  T
where T: Fn() 
{
    fn run(&self){
        serial_print!("{}....\t", core::any::type_name::<T>());
        self();
        serial_print!("[ok]\n")
    }
}

#[test_case]
fn tassert() {
    assert_eq!(1, 1);
}


