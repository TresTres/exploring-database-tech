
use std::io::prelude::*;
use std::fs::OpenOptions;
use std::path::PathBuf;



pub fn write_line_to_file(filepath: PathBuf, line: &str) -> Result<
    (),
    std::io::Error
> {
    /*
    Appends a line to a file in the given filepath.
    If the file does not exist, it creates the file.
    */

    if filepath.try_exists()? {
        println!("File found at {:?}", filepath);
    } else {
        println!("File not found at {:?}, creating...", filepath);
    }
    let mut file =  OpenOptions::new().create(true).append(true).open(filepath).expect("Failed to open file");
    writeln!(file, "{}", line).expect("Failed to write to file");
    Ok(())
}




#[cfg(test)]
mod tests;