use std::fs;
use std::io;
use std::path::Path;

// Make directory
fn create_directory(path: &str) -> io::Result<()> {
    fs::create_dir(path)?;
    Ok(())
}

// Remove directory
fn remove_directory(path: &str) -> io::Result<()> {
    fs::remove_dir(path)?;
    Ok(())
}

// Read in directory
fn list_directory_contents(path: &str) -> io::Result<Vec<String>> {
    let mut contents = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        contents.push(path.to_string_lossy().into_owned());
    }
    Ok(contents)
}

fn main() -> io::Result<()> {
    // Example

    let dir_path = "example_dir";

    // Make directory
    create_directory(dir_path)?;
    println!("Directory created: {}", dir_path);

    // Read in directory
    let contents = list_directory_contents(".")?;
    println!("Contents of current directory:");
    for item in contents {
        println!("{}", item);
    }

    // Remove directory
    remove_directory(dir_path)?;
    println!("Directory removed: {}", dir_path);

    Ok(())
}
