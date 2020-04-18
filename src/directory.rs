use regex::Regex;

use browseable::Browseable;
use file::File;
use filesystem_node::FilesystemNode;
use fs_archive::FSArchive;
use readable::Readable;

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

    fn node_map(&self) -> std::collections::HashMap<String, FilesystemNode> {
        let mut nodes: std::collections::HashMap<String, FilesystemNode> =
            std::collections::HashMap::new();

        for file in self.list_files() {
            nodes.insert(
                String::from(file.filename()),
                FilesystemNode::Readable(file.clone()),
            );
        }

        for dir in self.list_subdirectories() {
            nodes.insert(
                String::from(dir.name()),
                FilesystemNode::Browseable(dir.clone()),
            );
        }

        nodes
    }

    fn list_archives(&self) -> Vec<Box<dyn Browseable>> {
        let entries = std::fs::read_dir(&self.dirpath).unwrap();

        let mut archives: Vec<Box<dyn Browseable>> = vec![];

        for node in entries {
            let n = node.unwrap();

            if !n.path().is_dir() {
                if let Some(extension) = n.path().extension() {
                    if extension == "rar" {
                        let archive = FSArchive::new(n.path().to_str().unwrap());
                        archives.push(Box::new(archive));
                    } else if extension == "gz" {
                        let archive = FSArchive::new(n.path().to_str().unwrap());
                        archives.push(Box::new(archive));
                    }
                }
            }
        }

        archives
    }

    fn path(&self) -> &std::path::Path {
        std::path::Path::new(&self.dirpath)
    }
}

impl Browseable for Directory {
    fn clone(&self) -> Box<dyn Browseable> {
        Box::new(Directory::new(&self.dirpath))
    }

    fn name(&self) -> &str {
        self.path().file_stem().unwrap().to_str().unwrap()
    }

    fn list_subdirectories(&self) -> Vec<Box<dyn Browseable>> {
        let entries = std::fs::read_dir(&self.dirpath).unwrap();

        let mut subdirectories: Vec<Box<dyn Browseable>> = vec![];

        for node in entries {
            let n = node.unwrap();

            if n.path().is_dir() {
                subdirectories.push(Box::new(Directory::new(n.path().to_str().unwrap())));
            }
        }

        for archive in self.list_archives() {
            subdirectories.push(archive.clone())
        }

        subdirectories
    }

    fn list_files(&self) -> Vec<Box<dyn Readable>> {
        let entries = std::fs::read_dir(&self.dirpath).unwrap();

        // Regex for matching against multi-part RAR-files
        let re = Regex::new(r"^r\d{2}$").unwrap();

        let mut files: Vec<Box<dyn Readable>> = vec![];

        for node in entries {
            let n = node.unwrap();

            if !n.path().is_dir() {
                if let Some(extension) = n.path().extension() {
                    let ex = extension.to_str().unwrap();

                    if re.is_match(ex) {
                        // No-op
                    } else if extension != "rar" {
                        files.push(Box::new(File::new(n.path().to_str().unwrap())));
                    }
                } else {
                    files.push(Box::new(File::new(n.path().to_str().unwrap())));
                }
            }
        }

        files
    }

    fn get_node(&self, path: &str) -> Option<FilesystemNode> {
        if path == "/" {
            return Some(FilesystemNode::Browseable(self.clone()));
        }

        let fragments: Vec<&str> = path.split('/').filter(|x| x != &"").collect();

        match fragments.as_slice() {
            [] => None,

            [filename] => {
                let mut foo = self.node_map();
                let f = String::from(*filename);
                foo.remove(&f)
            }

            rest => {
                let first = rest.first().unwrap();
                let rest_path = &rest[1..].join("/");

                match self.get_subdirectory(first) {
                    Some(dir) => dir.get_node(rest_path),
                    None => None,
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempdir::TempDir;

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
}
