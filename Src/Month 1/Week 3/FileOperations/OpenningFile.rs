use std::fs::File;

fn main() -> std::io::Result<()> {
    // Open a file for reading
    let file = File::open("example.txt")?;

    // Create a new file for writing
    let new_file = File::create("new_file.txt")?;

    // Perform file operations here

    Ok(())
}
