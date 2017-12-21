use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use walkdir::{DirEntry, WalkDir};
use std::ptr;

use archive::Archive;
use node::Node;

pub struct DirectoryArchive {
    archives: HashMap<String, Arc<Archive>>,
}

fn is_archive(entry: &DirEntry) -> bool {
    if let Some(extension) = entry.path().extension() {
        return extension == "rar";
    } else {
        return false;
    }
}

impl DirectoryArchive {
    pub fn new(path: &str) -> DirectoryArchive {
        let walker = WalkDir::new(path).into_iter();
        let walker = walker.filter_map(|e| e.ok());
        let walker = walker.filter(|e| is_archive(e));

        let mut archives: HashMap<String, Arc<Archive>> = HashMap::new();

        for entry in walker {
            let file_stem = entry.path().file_stem().unwrap().to_str().unwrap();
            let path = entry.path().to_str().unwrap();

            let archive: Arc<Archive> = Arc::new(Archive::new(path));
            archives.insert(String::from(file_stem), archive);
        }

        return DirectoryArchive { archives: archives };
    }

    pub fn list_files_in_root(&self) -> Vec<Arc<Mutex<Node>>> {
        let mut nodes: Vec<Arc<Mutex<Node>>> = vec![];

        for name in self.archives.keys() {
            let directory_node = Node::new(name.clone(), ptr::null_mut(), name.clone(), 8192);
            let ptr = Arc::new(Mutex::new(directory_node));
            nodes.push(ptr);
        }

        return nodes;
    }

    pub fn get_node_for_path(&self, path: &str) -> Option<Arc<Mutex<Node>>> {
        // If it's in the _dict as a key, return a node saying it's a directory
        for (filename, archive) in &self.archives {
            let filename_with_leading_slash: String = format!("/{}", filename);

            if path == filename_with_leading_slash {
                let directory_node = Node::new(
                    String::from(""),
                    ptr::null_mut(),
                    filename_with_leading_slash,
                    8192,
                );
                let ptr = Arc::new(Mutex::new(directory_node));
                return Some(ptr);
            }

            if path.starts_with(&filename_with_leading_slash) {
                let subpath_for_archive: String =
                    path.replacen(&filename_with_leading_slash, "", 1);
                return archive.get_node_for_path(&subpath_for_archive);
            }
        }

        return None;
    }

    pub fn get_nodes_in_directory(&self, directory_prefix: &str) -> Vec<Arc<Mutex<Node>>> {
        let nodes: Vec<Arc<Mutex<Node>>> = vec![];

        for (filename, archive) in &self.archives {
            let mut filepath: String = String::from("/");
            filepath.push_str(&filename);

            if directory_prefix == filepath {
                return archive.get_nodes_in_directory("/");
            }
        }

        return nodes;
    }
}
