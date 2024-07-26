mod common;
mod kv;

fn main() {
    let filename = "foo.txt";
    let target_dir = "file_directory";
    let store = kv::KVStore::new(filename, target_dir);
    kv::write_value_to_store(store, "foo", "bar");
}


