use std::io::BufReader;
use std::fs::File; 
use super::*;

use tempfile::tempdir;


fn create_temp_file(filename: &str) -> Result<(tempfile::TempDir, PathBuf), std::io::Error> {
    let tempdir = tempdir()?;
    let expected_path = tempdir.path().join(filename);
    File::create(&expected_path)?;
    Ok((tempdir, expected_path))
}

#[test]
fn test_search_for_file_failure() {
    let filename = "not_found.txt";
    let dirname = "file_directory";
    let path = search_for_file(filename, dirname);
    assert_eq!(path, None);
}

#[test]
fn test_search_for_file_success() -> std::io::Result<()> {
    // make temp file
    let filename = "found.txt";
    let (tempdir, expected_path) = create_temp_file(filename)?;

    // search for file
    let path = search_for_file(filename, tempdir.path().to_str().unwrap());
    assert_eq!(path, Some(expected_path));

    // cleanup
    tempdir.close()?;
    Ok(())
}

#[test]
fn test_write_to_file_new_file() -> std::io::Result<()> {
    // make temp file
    let filename = "new.txt";
    let tempdir = tempdir()?;
    let expected_path = tempdir.path().join(filename);

    // write to file
    write_to_file(filename, tempdir.path().to_str().unwrap());

    // check file contents
    let file = File::open(&expected_path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    assert_eq!(lines.len(), 1);
    assert_eq!(lines, vec!["key: value"]);

    // cleanup
    tempdir.close()?;
    Ok(())
}


#[test]
fn test_write_to_file_existing_file() -> std::io::Result<()> {
    // make temp file
    let filename = "existing.txt";
    let (tempdir, expected_path) = create_temp_file(filename)?;
    
    // write to file
    std::fs::write(&expected_path, "key: value\n")?;
    write_to_file(filename, tempdir.path().to_str().unwrap());

    // check file contents
    let file = File::open(&expected_path)?;
    let reader = BufReader::new(file);
    let lines: Vec<String> = reader.lines().map(|l| l.unwrap()).collect();
    assert_eq!(lines.len(), 2);
    assert_eq!(lines, vec!["key: value", "key: value"]);

    // cleanup
    tempdir.close()?;
    Ok(())
}