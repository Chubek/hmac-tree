#![allow(unused)]

#[macro_use]
extern  crate lazy_static;

extern crate proc_macro;

pub mod sha512;
pub mod encoders;
mod utils;
pub mod hmac;
mod constants;
pub mod htree;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
