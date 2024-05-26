use super::*;

#[test]
fn test_search_for_file_failure() {
    let filename = "foobar.txt";
    let path = search_for_file(filename);
    assert_eq!(path, None);
}