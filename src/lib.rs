pub mod ast;
// pub mod codegen;
pub mod parse;

// use codegen::{SymbolGenerator, Wasm};

pub fn compile(source: &str) -> Vec<u8> {
    wat::parse_str(compile_wat(source)).unwrap()
}

pub fn compile_wat(source: &str) -> String {
    // let mut s = SymbolGenerator::new();
    let ast = parse::interpret(source);
    // println!("{:?}", ast);
    unimplemented!()
    // let wasm = ast.emit(&mut s);
    // println!("{}", wasm);
    // wasm
}
