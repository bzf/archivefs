extern crate archivefs;
extern crate tempdir;

use tempdir::TempDir;

use archivefs::{Browseable, FSArchive};

#[test]
fn test_name() {
    let tmp_dir = TempDir::new("test_archive_in_root_as_directory").unwrap();
    let archive_path = tmp_dir.path().join("single-level-single-file-archive.rar");
    std::fs::copy(
        std::env::current_dir()
            .unwrap()
            .join("tests/fixtures/single-level-single-file-archive.rar"),
        &archive_path,
    )
    .unwrap();

    let fs_archive = FSArchive::new(&archive_path.to_str().unwrap());

    assert_eq!(fs_archive.name(), "single-level-single-file-archive");
}

#[test]
fn test_simple_list_files() {
    let tmp_dir = TempDir::new("test_archive_in_root_as_directory").unwrap();
    let archive_path = tmp_dir.path().join("single-level-archive.gz");
    std::fs::copy(
        std::env::current_dir()
            .unwrap()
            .join("tests/fixtures/single-level-archive.gz"),
        &archive_path,
    )
    .unwrap();

    let fs_archive = FSArchive::new(&archive_path.to_str().unwrap());

    assert_eq!(fs_archive.name(), "single-level-archive");
    assert_eq!(
        fs_archive.list_subdirectories().len(),
        0,
        "should not show subdirectories"
    );
    assert_eq!(fs_archive.list_files().len(), 1, "should have files");

    let files = fs_archive.list_files();
    let filenames: Vec<&str> = files.iter().map(|x| x.filename()).collect();
    assert_eq!(
        filenames,
        vec!["hello.txt"],
        "should have one file named 'hello.txt'"
    );
}

#[test]
fn test_name_with_tar_gz_file_extension() {
    let tmp_dir = TempDir::new("test_name_with_tar_gz_file_extension").unwrap();
    let archive_path = tmp_dir.path().join("single-level-archive.tar.gz");
    std::fs::copy(
        std::env::current_dir()
            .unwrap()
            .join("tests/fixtures/single-level-archive.gz"),
        &archive_path,
    )
    .unwrap();

    let fs_archive = FSArchive::new(&archive_path.to_str().unwrap());

    assert_eq!(fs_archive.name(), "single-level-archive");
}

#[test]
fn test_name_with_zip_file_extension() {
    let tmp_dir = TempDir::new("test_name_with_zip_file_extension").unwrap();
    let archive_path = tmp_dir.path().join("single-level-archive.zip");
    std::fs::copy(
        std::env::current_dir()
            .unwrap()
            .join("tests/fixtures/single-level-archive.zip"),
        &archive_path,
    )
    .unwrap();

    let fs_archive = FSArchive::new(&archive_path.to_str().unwrap());

    assert_eq!(fs_archive.name(), "single-level-archive");
    assert_eq!(fs_archive.list_files().len(), 1, "should have files");

    let files = fs_archive.list_files();
    let filenames: Vec<&str> = files.iter().map(|x| x.filename()).collect();
    assert_eq!(
        filenames,
        vec!["hello.txt"],
        "should have one file named 'hello.txt'"
    );
}
