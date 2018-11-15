#![no_std]
extern crate embedded_hal as hal;
extern crate nb;

mod bindings;
mod ctypes;
pub mod leuart;
pub mod peripherals;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
