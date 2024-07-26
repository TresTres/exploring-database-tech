use std::collections::HashMap;
use std::io::BufRead as _;
use std::path::PathBuf;

use crate::common::file;

pub struct KVStore {
    filename: String,
    target_dir: String,
    file_length: u64,
    buffer: Vec<u8>,
    filepath: PathBuf,
    data: HashMap<String, String>,
}

impl KVStore {
    pub fn new(filename: &str, target_dir: &str) -> Self {
        KVStore {
            filename: filename.to_string(),
            target_dir: target_dir.to_string(),
            file_length: 100,
            buffer: Vec::new(),
            filepath: PathBuf::from(&format!("{}/{}", target_dir, filename)),
            data: HashMap::new(),
        }
    }


    pub fn load(&mut self) -> Result<(), std::io::Error> {
        let mut line = String::new();
        let mut buffer = file::read_file_to_buffer(&self.filepath);
        while buffer.read_line(&mut line)? > 0 {
            let parts: Vec<&str> = line.split(":").collect();
            self.data.insert(parts[0].to_string(), parts[1].to_string());
            line.clear();
        }
        Ok(())
    }

    pub fn get_key(&self, key: &str) -> &str {
        self.data.get(key).unwrap()
    }
}

pub fn write_value_to_store(store: &KVStore, key: &str, value: &str) -> Result<(), std::io::Error> {
    /*
    Writes a key-value pair to the store.
    */
    let line = format!("{}: {}", key, value);
    file::write_line_to_file(&store.filepath, &line)
}



#[cfg(test)]
mod tests;
