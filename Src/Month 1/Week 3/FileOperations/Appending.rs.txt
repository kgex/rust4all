use std::fs::{File, OpenOptions};
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut file = OpenOptions::new()
        .append(true)
        .open("existing_file.txt")?;

    let data = "Appended data\n";

    file.write_all(data.as_bytes())?;

    Ok(())
}
