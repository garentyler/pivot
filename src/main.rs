use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    // Read the source from a file.
    let mut src = String::new();
    File::open("test.pvt")?.read_to_string(&mut src)?;
    // Compile it
    let binary = pivot::compile(src);
    // Write it to a file.
    File::create("out.wasm")?.write_all(&binary)?;
    Ok(())
}
