extern crate wat;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let src = r#"function add(left, right) { return left + right; }"#;
    let ast = pivot::parse(src);
    // println!("{}", ast);
    let mut code = String::new();
    ast.emit(&mut code);
    println!("{}", code);
    let binary = wat::parse_str(code).unwrap();
    let mut file = File::create("out.wasm")?.write_all(&binary)?;
    Ok(())
}
