use std::fs::File;
use std::io::{self, Read};

fn main() -> io::Result<()> {
    let mut file = File::open("example.txt")?;
    let mut content = String::new();

    file.read_to_string(&mut content)?;

    println!("File content: {}", content);

    Ok(())
}
