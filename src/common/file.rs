
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::path::PathBuf;


fn search_for_file(filename: &str, dirname: &str) -> Option<PathBuf> {
    /*
    Searches for a filepath within the given dirname, and if found returns the Path object
    wrapped in Some
    if not found, returns None
    */
    let filepath = PathBuf::from(&format!("{}/{}", dirname, filename));
    let result = filepath.try_exists().unwrap();
    if result {
        return Some(filepath);
    }
    None
}

pub fn write_to_file(filename: &str) {
    let target_dir = "file_directory";
    let path = search_for_file(filename, target_dir);
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