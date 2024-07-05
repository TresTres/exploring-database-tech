use std::io::BufReader;
use std::fs::File; 
use super::*;

#[test]
fn test_set_key() -> std::io::Result<()> {

    // create a database
    // call add key method
    // verify that the key was added to the underlying file 

    let filename = "foo.kv";
    let target_dir = "file_directory";
    let db = KVStore::get_or_create(filename, target_dir);
    db.set_key("10001", "{\"name\": \"John Doe\"}")?;
    let log = db.get_log();
    assert_eq!(log.len(), 1);
    asssert_eq!(log[0], "10001: {\"name\": \"John Doe\"}");
    Ok(())
}