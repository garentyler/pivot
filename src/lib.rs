extern crate regex;
extern crate wat;

pub mod ast;
pub mod codegen;
pub mod parse;
use codegen::{SymbolGenerator, Wasm};

pub fn compile<T: Into<String>>(src: T) -> Vec<u8> {
    wat::parse_str(compile_wat(src)).unwrap()
}

pub fn compile_wat<T: Into<String>>(src: T) -> String {
    let mut s = SymbolGenerator::new();
    parse::parse(src).emit(&mut s)
}
