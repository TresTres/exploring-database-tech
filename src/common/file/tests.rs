use std::fs::File;
use super::*;

use tempfile::tempdir;

#[test]
fn test_search_for_file_failure() {
    let filename = "not_found.txt";
    let dirname = "file_directory";
    let path = search_for_file(filename, dirname);
    assert_eq!(path, None);
}

#[test]
fn test_search_for_file_success() -> std::io::Result<()> {
    let filename = "found.txt";

    // make fake file
    let tempdir = tempdir()?;
    let expected_path = tempdir.path().join(filename);
    File::create(&expected_path)?;

    // search for file
    let path = search_for_file(filename, tempdir.path().to_str().unwrap());
    assert_eq!(path, Some(expected_path));

    // cleanup
    tempdir.close()?;
    Ok(())
}