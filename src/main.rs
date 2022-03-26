use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // Read the source from a file.
    let source = std::fs::read_to_string("test.pvt").unwrap();
    // Compile it
    let _value = pivot::parse::interpret(&source);
    // let binary = pivot::compile(&source);
    // Write it to a file.
    // File::create("out.wasm")?.write_all(&binary)?;
    Ok(())
}
