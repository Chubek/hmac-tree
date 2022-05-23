#![allow(unused)]

#[macro_use]
extern crate lazy_static;

extern crate proc_macro;

mod constants;
pub mod encoders;
pub mod hmac;
pub mod htree;
pub mod sha512;
mod utils;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
