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

    pub fn list_archives(&self) -> Vec<Box<dyn Browseable>> {
        let entries = std::fs::read_dir(&self.dirpath).unwrap();

        let mut archives: Vec<Box<dyn Browseable>> = vec![];

        for node in entries {
            let n = node.unwrap();

            if !n.path().is_dir() {
                if let Some(extension) = n.path().extension() {
                    if extension == "rar" {
                        let archive = FSArchive::new(n.path().to_str().unwrap());
                        archives.push(Box::new(archive));
                    } else if extension == "zip" {
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
                    } else if extension != "rar" && extension != "gz" {
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
