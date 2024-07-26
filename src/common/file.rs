
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

pub fn write_line_to_file(filename: &str, target_dir: &str, line: &str) -> Result<
    (),
    std::io::Error
> {
    let path = search_for_file(filename, target_dir);
    let mut open_options = OpenOptions::new();
    open_options.create(true).append(true).read(true);
    let file_result: Result<std::fs::File, std::io::Error>; 
    match path {
        Some(path) => {
            println!("File found: {:?}", path);
            file_result = open_options.open(path);
        }
        None => {
            println!("File not found, creating file...");
            file_result = open_options.open(&format!("{}/{}", target_dir, filename));
        }
    }
    let mut file = file_result?;
    writeln!(file, "{}", line).expect("Failed to write to file");
    Ok(())
}




#[cfg(test)]
mod tests;