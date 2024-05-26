
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::path::PathBuf;


fn search_for_file(filename: &str) -> Option<PathBuf> {
    /*
    Searches for a filepath, and if found returns the Path object
    if not found, returns false
    */
    let filepath = PathBuf::from(&format!("file_directory/{}", filename));
    let result = filepath.try_exists().unwrap();
    if result {
        return Some(filepath);
    }
    None
}

pub fn write_to_file(filename: &str) {
    let path = search_for_file(filename);
    let mut open_options = OpenOptions::new();
    open_options.create(true).append(true).read(true);
    let mut file; 
    match path {
        Some(path) => {
            println!("File found: {:?}", path);
            file = open_options.open(path).unwrap();
        }
        None => {
            println!("File not found, creating file...");
            file = open_options.open(&format!("file_directory/{}", filename)).unwrap();
        }
    }
    writeln!(file, "key: value").expect("Failed to write to file");

}


#[cfg(test)]
mod tests;