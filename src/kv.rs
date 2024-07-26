
use crate::common::file;



pub struct KVStore {
    filename: String,
    target_dir: String,
    file_length: u64,
    buffer: Vec<u8>,
}


impl KVStore {
    pub fn new(filename: &str, target_dir: &str) -> Self {
        KVStore {
            filename: filename.to_string(),
            target_dir: target_dir.to_string(),
            file_length: 100, 
            buffer: Vec::new(),
        }
    }
}


pub fn write_value_to_store(store: KVStore, key: &str, value: &str) -> Result<
    (),
    std::io::Error
> {

    let line = format!("{}: {}", key, value);
    file::write_line_to_file(store.filename.as_str(), store.target_dir.as_str(), line.as_str())
}



#[cfg(test)]
mod tests;