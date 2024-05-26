
use std::fs::File;
use std::path::Path;
use std::io::prelude::*;

fn search_for_file(filename: String) -> Option<&Path> {
    /*
    Searches for a filepath, and if found returns the Path object
    if not found, returns false
    */
    let filepath: &Path = Path::new("file_directory/{filename}");
    let result: bool = filepath.try_exists().unwrap();
    if result {
        return Some(filepath);
    }
    None
}

fn write_to_file() -> std::io::Result<()> {
    let mut file = File::create("file_directory/foo.txt")?;
    file.write_all(b"Hello, world!")?;
    Ok(())
}

fn main() {
    
}





// simplest key value store database ever 
// find a file, if it doesn't exist or the current file is full, create one
// write a key and value store to the file, then search for it and print it out