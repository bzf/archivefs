extern crate archivefs;
extern crate tempdir;

use std::io::Write;
use tempdir::TempDir;

// use archivefs::readable::Readable;
use archivefs::{File, Readable};

#[test]
fn test_file_size() {
    let tmp_dir = TempDir::new("example").unwrap();

    let file_path = tmp_dir.path().join("my-temporary-note.txt");
    let mut tmp_file = std::fs::File::create(&file_path).unwrap();

    let content = "Brian was here. Briefly.";
    writeln!(tmp_file, "{}", content).unwrap();

    let file = File::new(file_path.as_path().to_str().unwrap());
    assert_eq!((content.len() + 1) as u64, file.size());
}

#[test]
fn test_file_name() {
    let tmp_dir = TempDir::new("example").unwrap();

    let file_path = tmp_dir.path().join("my-temporary-note.txt");
    let mut tmp_file = std::fs::File::create(&file_path).unwrap();

    writeln!(tmp_file, "Hello").unwrap();

    let file = File::new(file_path.as_path().to_str().unwrap());
    assert_eq!(file.filename(), "my-temporary-note.txt");
}
