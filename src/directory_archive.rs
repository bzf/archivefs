use std::collections::HashMap;
use std::rc::Rc;
use walkdir::{DirEntry, WalkDir};
use std::ptr;

use archive::Archive;
use node::Node;

pub struct DirectoryArchive {
    archives: HashMap<String, Rc<Archive>>,
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

        let mut archives: HashMap<String, Rc<Archive>> = HashMap::new();

        for entry in walker {
            let file_stem = entry.path().file_stem().unwrap().to_str().unwrap();
            let path = entry.path().to_str().unwrap();

            let archive: Rc<Archive> = Rc::new(Archive::new(path));
            archives.insert(String::from(file_stem), archive);
        }

        return DirectoryArchive { archives: archives };
    }

    pub fn list_files_in_root(&self) -> Vec<String> {
        let mut names: Vec<String> = vec![];

        for name in self.archives.keys() {
            names.push(name.clone());
        }

        return names;
    }

    // pub fn get_node_for_path(&self,
    pub fn get_node_for_path(&self, path: &str) -> Option<Rc<Node>> {
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
                let ptr = Rc::new(directory_node);
                return Some(ptr);
            }

            if path.starts_with(&filename_with_leading_slash) {
                let subpath_for_archive: String = path.replace(&filename_with_leading_slash, "");
                return archive.get_node_for_path(&subpath_for_archive);
            }
        }

        return None;
    }

    pub fn get_nodes_in_directory(&self, directory_prefix: &str) -> Vec<Rc<Node>> {
        let nodes: Vec<Rc<Node>> = vec![];

        for (filename, archive) in &self.archives {
            if directory_prefix == filename {
                return archive.get_nodes_in_directory("/");
            }
        }

        return nodes;
    }
}
