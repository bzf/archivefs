use std::io::Write;
use tempdir::TempDir;

struct File {
    filepath: String,
}

impl File {
    pub fn new(filepath: &str) -> File {
        File {
            filepath: String::from(filepath),
        }
    }

    pub fn filename(&self) -> &str {
        self.path().file_name().unwrap().to_str().unwrap()
    }

    pub fn size(&self) -> u64 {
        let metadata = std::fs::metadata(&self.filepath).unwrap();
        metadata.len()
    }

    fn path(&self) -> &std::path::Path {
        std::path::Path::new(&self.filepath)
    }
}

#[test]
fn test_file_size() {
    let tmp_dir = TempDir::new("example").unwrap();

    let file_path = tmp_dir.path().join("my-temporary-note.txt");
    let mut tmp_file = std::fs::File::create(&file_path).unwrap();

    let content = "Brian was here. Briefly.";
    writeln!(tmp_file, "{}", content).unwrap();

    println!("{:?}", file_path);
    let file = File::new(file_path.as_path().to_str().unwrap());
    assert_eq!((content.len() + 1) as u64, file.size());
}

#[test]
fn test_file_name() {
    let tmp_dir = TempDir::new("example").unwrap();

    let file_path = tmp_dir.path().join("my-temporary-note.txt");
    let mut tmp_file = std::fs::File::create(&file_path).unwrap();

    writeln!(tmp_file, "Hello").unwrap();

    println!("{:?}", file_path);
    let file = File::new(file_path.as_path().to_str().unwrap());
    assert_eq!(file.filename(), "my-temporary-note.txt");
}

#[derive(Debug)]
struct Directory {
    dirpath: String,
}

impl Directory {
    pub fn new(dirpath: &str) -> Directory {
        Directory {
            dirpath: String::from(dirpath),
        }
    }

    pub fn clone(&self) -> Directory {
        Directory::new(&self.dirpath)
    }

    pub fn name(&self) -> &str {
        self.path().file_stem().unwrap().to_str().unwrap()
    }

    pub fn size(&self) -> u64 {
        4096
    }

    pub fn list_files(&self) -> Vec<File> {
        let entries = std::fs::read_dir(&self.dirpath).unwrap();

        entries
            .filter(|e| !e.as_ref().unwrap().path().is_dir())
            .map(|e| File::new(&e.unwrap().path().to_str().unwrap()))
            .collect()
    }

    pub fn list_subdirectories(&self) -> Vec<Directory> {
        let entries = std::fs::read_dir(&self.dirpath).unwrap();

        entries
            .filter(|e| e.as_ref().unwrap().path().is_dir())
            .map(|e| Directory::new(&e.unwrap().path().to_str().unwrap()))
            .collect()
    }

    pub fn get_subdirectory(&self, name: &str) -> Option<Directory> {
        let subdirectories = self.list_subdirectories();
        let subdirectory: Option<&Directory> = subdirectories.iter().find(|dir| dir.name() == name);

        match subdirectory {
            None => None,
            Some(dir) => Some(dir.clone()),
        }
    }

    fn path(&self) -> &std::path::Path {
        std::path::Path::new(&self.dirpath)
    }
}

// #[test]
// fn test_directory_size_works() {
// }

#[derive(Debug)]
struct Filesystem {
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
