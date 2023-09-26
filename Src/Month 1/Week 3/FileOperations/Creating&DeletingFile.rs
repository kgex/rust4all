use std::fs;

fn main() -> std::io::Result<()> {
    // Create a directory
    fs::create_dir("my_directory")?;

    // Delete a directory
    fs::remove_dir("my_directory")?;

    // Delete a file
    fs::remove_file("file_to_delete.txt")?;

    Ok(())
}
