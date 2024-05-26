mod common;
// simplest key value store database ever
// find a file, if it doesn't exist or the current file is full, create one
// write a key and value store to the file, then search for it and print it out

fn main() {
    let filename = "foo.txt";
    common::file::write_to_file(filename);
}


