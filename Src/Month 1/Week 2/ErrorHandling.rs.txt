use std::fs::File;
use std::io::{self, Read};

fn read_file() -> Result<(), io::Error> {
    let mut file = File::open("example.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    println!("File content: {}", content);
    Ok(())
}

fn main() -> Result<(), io::Error> {
    read_file()?;
    Ok(())
}
