#![allow(unused_variables)]
#![allow(non_snake_case)]
#![allow(dead_code)]

extern crate regex;
extern crate wat;

pub mod ast;
pub mod parse;

pub fn compile<T: Into<String>>(src: T) -> Vec<u8> {
    wat::parse_str(compile_wat(src)).unwrap()
}

pub fn compile_wat<T: Into<String>>(src: T) -> String {
    parse::parse(src).emit()
}
