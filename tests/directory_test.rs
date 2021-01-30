extern crate archivefs;
extern crate tempdir;

use std::io::Write;
use tempdir::TempDir;

use archivefs::{Browseable, Directory, FilesystemNode};

#[test]
fn test_getting_subdirectory_node() {
    let tmp_dir = TempDir::new("test_getting_subdirectory_node").unwrap();
    std::fs::create_dir(tmp_dir.path().join("hello")).unwrap();

    let directory = Directory::new(tmp_dir.path().to_str().unwrap());

    match directory.get_node("/hello") {
        Some(FilesystemNode::Browseable(dir)) => assert_eq!(dir.name(), "hello"),
        _ => unreachable!(),
    }
}

#[test]
fn test_getting_node_directory_in_subdirectory() {
    let tmp_dir = TempDir::new("test_getting_node_directory_in_subdirectory").unwrap();
    std::fs::create_dir(tmp_dir.path().join("hello")).unwrap();
    std::fs::create_dir(tmp_dir.path().join("hello").join("world")).unwrap();

    let directory = Directory::new(tmp_dir.path().to_str().unwrap());

    match directory.get_node("/hello/world") {
        Some(FilesystemNode::Browseable(dir)) => assert_eq!(dir.name(), "world"),
        _ => unreachable!(),
    }
}

#[test]
fn test_getting_node_file_in_subdirectory() {
    let tmp_dir = TempDir::new("test_getting_node_directory_in_subdirectory").unwrap();
    std::fs::create_dir(tmp_dir.path().join("hello")).unwrap();
    std::fs::create_dir(tmp_dir.path().join("hello").join("world")).unwrap();

    let file_path = tmp_dir.path().join("hello/world/foo.txt");
    let mut tmp_file = std::fs::File::create(file_path).unwrap();
    writeln!(tmp_file, "foo").unwrap();

    let directory = Directory::new(tmp_dir.path().to_str().unwrap());

    match directory.get_node("/hello/world/foo.txt") {
        Some(FilesystemNode::Readable(file)) => assert_eq!(file.filename(), "foo.txt"),
        _ => unreachable!(),
    }
}

#[test]
fn test_getting_root_file_node() {
    let tmp_dir = TempDir::new("test_getting_root_file_node").unwrap();

    let file_path = tmp_dir.path().join("foo.txt");
    let mut tmp_file = std::fs::File::create(file_path).unwrap();
    writeln!(tmp_file, "foo").unwrap();

    let directory = Directory::new(tmp_dir.path().to_str().unwrap());

    match directory.get_node("/foo.txt") {
        Some(FilesystemNode::Readable(file)) => assert_eq!(file.filename(), "foo.txt"),
        _ => unreachable!(),
    }
}

#[test]
fn test_listing_all_nodes() {
    let tmp_dir = TempDir::new("example").unwrap();

    let file_path = tmp_dir.path().join("foo.txt");
    let mut tmp_file = std::fs::File::create(file_path).unwrap();
    writeln!(tmp_file, "foo").unwrap();

    std::fs::create_dir(tmp_dir.path().join("hello")).unwrap();

    let directory = Directory::new(tmp_dir.path().to_str().unwrap());

    assert_eq!(directory.list_nodes().len(), 2);
    match directory.list_nodes().first() {
        Some(FilesystemNode::Readable(file)) => assert_eq!(file.filename(), "foo.txt"),
        Some(FilesystemNode::Browseable(dir)) => assert_eq!(dir.name(), "hello"),
        _ => unreachable!(),
    }

    match directory.list_nodes().last() {
        Some(FilesystemNode::Readable(file)) => assert_eq!(file.filename(), "foo.txt"),
        Some(FilesystemNode::Browseable(dir)) => assert_eq!(dir.name(), "hello"),
        _ => unreachable!(),
    }
}

#[test]
fn test_listing_tar_gz_archive() {
    let tmp_dir = TempDir::new("test_listing_tar_gz_archive").unwrap();
    let archive_path = tmp_dir.path().join("single-level-archive.tar.gz");
    std::fs::copy(
        std::env::current_dir()
            .unwrap()
            .join("tests/fixtures/single-level-archive.gz"),
        &archive_path,
    )
    .unwrap();

    let directory = Directory::new(tmp_dir.path().to_str().unwrap());

    assert_eq!(directory.list_files().len(), 0,);
    assert_eq!(directory.list_subdirectories().len(), 1);
    assert_eq!(directory.list_archives().len(), 1);

    match directory.get_node("/single-level-archive") {
        Some(FilesystemNode::Browseable(archive)) => {
            assert_eq!(archive.name(), "single-level-archive");
        }
        _ => unreachable!(),
    }
}

#[test]
fn test_listing_archives() {
    let tmp_dir = TempDir::new("test_listing_archives").unwrap();

    let archive_path = tmp_dir.path().join("single-level-single-file-archive.rar");
    std::fs::copy(
        std::env::current_dir()
            .unwrap()
            .join("tests/fixtures/single-level-single-file-archive.rar"),
        &archive_path,
    )
    .unwrap();
    std::fs::File::create(tmp_dir.path().join("single-level-single-file-archive.r00")).unwrap();
    std::fs::File::create(tmp_dir.path().join("single-level-single-file-archive.r01")).unwrap();
    std::fs::File::create(tmp_dir.path().join("single-level-single-file-archive.r02")).unwrap();
    std::fs::File::create(tmp_dir.path().join("single-level-single-file-archive.r03")).unwrap();

    let directory = Directory::new(tmp_dir.path().to_str().unwrap());

    assert_eq!(directory.list_files().len(), 0);
    assert_eq!(directory.list_subdirectories().len(), 1);
    assert_eq!(directory.list_archives().len(), 1);

    match directory.list_archives().first() {
        Some(archive) => {
            assert_eq!(archive.name(), "single-level-single-file-archive");
        }
        _ => unreachable!(),
    }
}

#[test]
fn test_listing_zip_archive() {
    let tmp_dir = TempDir::new("test_listing_zip_archive").unwrap();

    let archive_path = tmp_dir.path().join("single-level-archive.zip");
    std::fs::copy(
        std::env::current_dir()
            .unwrap()
            .join("tests/fixtures/single-level-archive.zip"),
        &archive_path,
    )
    .unwrap();

    let directory = Directory::new(tmp_dir.path().to_str().unwrap());

    assert_eq!(directory.list_subdirectories().len(), 1);
    assert_eq!(directory.list_archives().len(), 1);

    match directory.list_archives().first() {
        Some(archive) => {
            assert_eq!(archive.name(), "single-level-archive");
        }
        _ => unreachable!(),
    }
}
