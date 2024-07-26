mod common;
mod kv;

fn main() {
    let filename = "foo.txt";
    let target_dir = "file_directory";
    let mut store = kv::KVStore::new(filename, target_dir);
    kv::write_value_to_store(&store, "foo", "batz").expect("Failed to write to store");
    store.load().expect("Failed to load store");
    println!("{:?}", store.get_key("foo"));
}


