use std::io::Write;
use tempdir::TempDir;

use directory::Directory;
use file::File;

#[derive(Debug)]
pub struct Filesystem {
    path: String,
}

impl Filesystem {
    pub fn new(path: &str) -> Filesystem {
        Filesystem {
            path: String::from(path),
        }
    }

    pub fn list_files(&self, path: &str) -> Vec<File> {
        if (path == "/") {
            self.root_directory().list_files()
        } else {
            match self.get_directory(&path) {
                Some(dir) => dir.list_files(),
                None => vec![],
            }
        }
    }

    fn get_file(&self, path: &str) -> Option<File> {
        let fragments: Vec<&str> = path.split('/').collect();

        match fragments.as_slice() {
            [directories, filename] => {
                let directory = self.get_directory(directories);
                directory.map(|dir| dir.get_file(filename)).unwrap_or(None)
            }
            [filename] => self.root_directory().get_file(&filename),
            _ => None,
        }
    }

    fn get_directory(&self, path: &str) -> Option<Directory> {
        let fragments = path.split('/');
        let vec: Vec<&str> = fragments.collect();

        let mut current_directory = self.root_directory();

        for directory in vec {
            match current_directory.get_subdirectory(directory) {
                None => return None,
                Some(dir) => current_directory = dir,
            }
        }

        Some(current_directory)
    }

    fn root_directory(&self) -> Directory {
        Directory::new(&self.path)
    }
}

#[test]
fn test_getting_file_in_root() {
    let tmp_dir = TempDir::new("example").unwrap();

    let file_path = tmp_dir.path().join("my-temporary-note.txt");
    let mut tmp_file = std::fs::File::create(file_path).unwrap();
    writeln!(tmp_file, "Brian was here. Briefly.").unwrap();

    let filesystem = Filesystem::new(tmp_dir.path().to_str().unwrap());

    match filesystem.get_file("my-temporary-note.txt") {
        Some(file) => {
            assert_eq!(file.filename(), "my-temporary-note.txt");
            assert_eq!(file.size(), 25);
        }
        _ => assert!(false, "Expected a file"),
    }
}

#[test]
fn test_getting_file_in_subdirectory() {
    let tmp_dir = TempDir::new("example").unwrap();
    let sub_dir_path = tmp_dir.path().join("subdir");
    std::fs::create_dir(&sub_dir_path).unwrap();

    let file_path = sub_dir_path.join("foo.txt");
    let mut tmp_file = std::fs::File::create(file_path).unwrap();
    writeln!(tmp_file, "foo").unwrap();

    let filesystem = Filesystem::new(tmp_dir.path().to_str().unwrap());

    match filesystem.get_file("subdir/foo.txt") {
        Some(file) => {
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
    let files_in_root: Vec<File> = filesystem.list_files("/");
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
    let files_in_root: Vec<File> = filesystem.list_files("subdir");
    assert_eq!(files_in_root.len(), 1);

    let filenames_in_root: Vec<&str> = files_in_root.iter().map(|x| x.filename()).collect();
    assert_eq!(filenames_in_root, vec!["foo.txt"]);
}
