use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut file = File::create("new_file.txt")?;
    let data = "Hello, Rust!\n";

    file.write_all(data.as_bytes())?;

    Ok(())
}
