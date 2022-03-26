use std::{fs::File, io::Write};

fn main() -> std::io::Result<()> {
    // Read the source from a file.
    let source = std::fs::read_to_string("test.pvt").unwrap();
    let code = pivot::compile(&source).unwrap();
    // Write it to a file.
    File::create("out.bf")?.write_all(code.as_bytes())?;
    Ok(())
}
