extern crate archivefs;
extern crate tempdir;

use std::io::Write;
use tempdir::TempDir;

use archivefs::{Filesystem, FilesystemNode, Readable};

#[test]
fn test_getting_root_directory_node() {
    let tmp_dir = TempDir::new("example").unwrap();
    let sub_dir_path = tmp_dir.path().join("subdir");
    std::fs::create_dir(&sub_dir_path).unwrap();

    let file_path = sub_dir_path.join("foo.txt");
    let mut tmp_file = std::fs::File::create(file_path).unwrap();
    writeln!(tmp_file, "foo").unwrap();

    let filesystem = Filesystem::new(tmp_dir.path().to_str().unwrap());

    match filesystem.get_node("/") {
        Some(FilesystemNode::Browseable(dir)) => {
            assert_eq!(dir.name(), "example");
        }
        _ => assert!(false, "Expected root directory"),
    }
}

#[test]
fn test_getting_root_file_node() {
    let tmp_dir = TempDir::new("example").unwrap();

    let file_path = tmp_dir.path().join("foo.txt");
    let mut tmp_file = std::fs::File::create(file_path).unwrap();
    writeln!(tmp_file, "foo").unwrap();

    let filesystem = Filesystem::new(tmp_dir.path().to_str().unwrap());

    match filesystem.get_node("/foo.txt") {
        Some(FilesystemNode::Readable(file)) => {
            assert_eq!(file.filename(), "foo.txt");
            assert_eq!(file.size(), 4);
        }
        _ => assert!(false, "Expected a file"),
    }
}

#[test]
fn test_lists_files_in_root_path() {
    let tmp_dir = TempDir::new("example").unwrap();

    let file_path = tmp_dir.path().join("my-temporary-note.txt");
    let mut tmp_file = std::fs::File::create(file_path).unwrap();
    writeln!(tmp_file, "Brian was here. Briefly.").unwrap();

    let filesystem = Filesystem::new(tmp_dir.path().to_str().unwrap());
    let files_in_root: Vec<Box<dyn Readable>> = filesystem.list_files("/");
    assert_eq!(files_in_root.len(), 1);

    let filenames_in_root: Vec<&str> = files_in_root.iter().map(|x| x.filename()).collect();
    assert_eq!(filenames_in_root, vec!["my-temporary-note.txt"]);
}

#[test]
fn test_lists_files_in_relative_path() {
    let tmp_dir = TempDir::new("example").unwrap();
    let sub_dir_path = tmp_dir.path().join("subdir");
    std::fs::create_dir(&sub_dir_path).unwrap();

    let file_path = sub_dir_path.join("foo.txt");
    let mut tmp_file = std::fs::File::create(file_path).unwrap();
    writeln!(tmp_file, "foo").unwrap();

    let filesystem = Filesystem::new(tmp_dir.path().to_str().unwrap());
    let files_in_root: Vec<Box<dyn Readable>> = filesystem.list_files("subdir");
    assert_eq!(files_in_root.len(), 1);

    let filenames_in_root: Vec<&str> = files_in_root.iter().map(|x| x.filename()).collect();
    assert_eq!(filenames_in_root, vec!["foo.txt"]);
}

#[test]
fn test_archive_in_root_as_directory() {
    let tmp_dir = TempDir::new("test_archive_in_root_as_directory").unwrap();
    std::fs::copy(
        std::env::current_dir()
            .unwrap()
            .join("tests/fixtures/single-level-single-file-archive.rar"),
        tmp_dir.path().join("single-level-single-file-archive.rar"),
    )
    .unwrap();

    let filesystem = Filesystem::new(tmp_dir.path().to_str().unwrap());
    let root_directory = filesystem.get_directory("/").unwrap();

    assert_eq!(
        root_directory.list_files().len(),
        0,
        "should not have any files"
    );

    let subdirectories = root_directory.list_subdirectories();
    assert_eq!(subdirectories.len(), 1, "should have a directory");
    let subdirectory_names: Vec<&str> = subdirectories.iter().map(|x| x.name()).collect();
    assert_eq!(subdirectory_names, vec!["single-level-single-file-archive"]);
}
