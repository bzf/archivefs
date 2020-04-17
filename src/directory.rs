use std::io::Write;
use tempdir::TempDir;

use file::File;
use filesystem_node::FilesystemNode;

#[derive(Debug)]
pub struct Directory {
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

    pub fn get_file(&self, filename: &str) -> Option<File> {
        let files = self.list_files();

        match files.iter().find(|file| file.filename() == filename) {
            None => None,
            Some(file) => Some(file.clone()),
        }
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

    pub fn get_node(&self, path: &str) -> Option<FilesystemNode> {
        let fragments: Vec<&str> = path.split('/').filter(|x| x != &"").collect();

        match fragments.as_slice() {
            [filename] => {
                let mut foo = self.node_map();
                let f = String::from(*filename);
                foo.remove(&f)
            }
            _ => None,
        }
    }

    fn node_map(&self) -> std::collections::HashMap<String, FilesystemNode> {
        let mut nodes: std::collections::HashMap<String, FilesystemNode> =
            std::collections::HashMap::new();

        for file in self.list_files() {
            nodes.insert(
                String::from(file.filename()),
                FilesystemNode::File(file.clone()),
            );
        }

        for dir in self.list_subdirectories() {
            nodes.insert(
                String::from(dir.name()),
                FilesystemNode::Directory(dir.clone()),
            );
        }

        nodes
    }

    pub fn list_nodes(&self) -> Vec<FilesystemNode> {
        let mut files: Vec<FilesystemNode> = self
            .list_files()
            .iter()
            .map(|file| FilesystemNode::File(file.clone()))
            .collect();

        let subdirectories: Vec<FilesystemNode> = self
            .list_subdirectories()
            .iter()
            .map(|directory| FilesystemNode::Directory(directory.clone()))
            .collect();

        files.extend(subdirectories);
        files
    }

    fn path(&self) -> &std::path::Path {
        std::path::Path::new(&self.dirpath)
    }
}

#[test]
fn test_getting_subdirectory_node() {
    let tmp_dir = TempDir::new("test_getting_subdirectory_node").unwrap();
    std::fs::create_dir(tmp_dir.path().join("hello")).unwrap();

    let directory = Directory::new(tmp_dir.path().to_str().unwrap());

    match directory.get_node("/hello") {
        Some(FilesystemNode::Directory(dir)) => assert_eq!(dir.name(), "hello"),
        _ => unreachable!(),
    }
}

// TODO: Fetch file is subdirectory
// TODO: Fetch directory is subdirectory

#[test]
fn test_getting_root_file_node() {
    let tmp_dir = TempDir::new("test_getting_root_file_node").unwrap();

    let file_path = tmp_dir.path().join("foo.txt");
    let mut tmp_file = std::fs::File::create(file_path).unwrap();
    writeln!(tmp_file, "foo").unwrap();

    let directory = Directory::new(tmp_dir.path().to_str().unwrap());

    match directory.get_node("/foo.txt") {
        Some(FilesystemNode::File(file)) => assert_eq!(file.filename(), "foo.txt"),
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
        Some(FilesystemNode::File(file)) => assert_eq!(file.filename(), "foo.txt"),
        Some(FilesystemNode::Directory(dir)) => assert_eq!(dir.name(), "hello"),
        _ => unreachable!(),
    }

    match directory.list_nodes().last() {
        Some(FilesystemNode::File(file)) => assert_eq!(file.filename(), "foo.txt"),
        Some(FilesystemNode::Directory(dir)) => assert_eq!(dir.name(), "hello"),
        _ => unreachable!(),
    }
}
