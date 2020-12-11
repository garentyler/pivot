extern crate wat;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let src = r#"
        function a(num) {
            return num;
        }
        function main(num) {
            var amt = a(2);
            return num + amt;
        }"#;
    let ast = pivot::parse(src);
    println!("{}", ast);
    let code = ast.emit();
    println!("{}", code);
    let binary = wat::parse_str(code).unwrap();
    File::create("out.wasm")?.write_all(&binary)?;
    Ok(())
}
