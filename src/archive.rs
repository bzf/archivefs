use std::collections::HashMap;
use std::os::raw::c_char;
use std::ffi::CStr;
use std::ptr;

use utils;
use ffi;
use node::Node;

pub struct Archive {
    path: String,
    files: HashMap<String, Node>,
}

impl Archive {
    pub fn new(path: &str) -> Archive {
        let path: String = String::from(path);
        println!("Creating rust Archive with '{}'", path);
        let mut files: HashMap<String, Node> = HashMap::new();

        let archive: *mut ffi::Archive = unsafe { ffi::archive_read_new() };
        unsafe {
            ffi::archive_read_support_filter_all(archive);
            ffi::archive_read_support_format_all(archive);
            ffi::archive_open_and_read_from_path(&path, archive, 10240);
        };

        let mut archive_entry: *mut ffi::ArchiveEntry = ptr::null_mut();
        loop {
            if unsafe { ffi::archive_read_next_header(archive, &mut archive_entry) != 0 } {
                break;
            }

            let cloned_entry: *mut ffi::ArchiveEntry =
                unsafe { ffi::archive_entry_clone(archive_entry) };
            let archive_pathname: *const c_char =
                unsafe { ffi::archive_entry_pathname(archive_entry) };
            let archive_path = unsafe { CStr::from_ptr(archive_pathname) };
            let archive_pathname: String = String::from(archive_path.to_str().unwrap());

            let node: Node = Node::new(
                String::from(path.clone()),
                cloned_entry,
                archive_pathname.clone(),
                8192,
            );

            let mut archive_pathname = utils::correct_path(archive_pathname);
            archive_pathname.insert(0, '/');
            files.insert(archive_pathname, node);

            unsafe { ffi::archive_read_data_skip(archive) };
        }

        unsafe { ffi::archive_read_free(archive) };

        return Archive {
            path: path,
            files: files,
        };
    }

    pub fn list_files_in_root(&self) -> Vec<String> {
        let mut archive: Vec<String> = vec![];

        for filepath in self.files.keys() {
            archive.push(filepath.clone());
        }

        return archive;
    }

    pub fn get_node_for_path(&self, path: &str) -> Option<Node> {
        for (filepath, node) in &self.files {
            if filepath == path {
                return Some(node.clone());
            }
        }

        return None;
    }

    pub fn get_nodes_in_directory(&self, directory_prefix: &str) -> Vec<Node> {
        let mut nodes: Vec<Node> = vec![];

        for (filepath, node) in &self.files {
            if filepath.len() > directory_prefix.len() {
                continue;
            }

            let mut path_with_directory_prefix: String = filepath.replace(directory_prefix, "");

            // Remove any leading slashes
            if path_with_directory_prefix.starts_with("/") {
                path_with_directory_prefix.remove(0);
            }

            // If the folder doesn't end on "/", show it in the directory
            if !path_with_directory_prefix.ends_with("/") {
                nodes.push(node.clone());
            }
        }

        return nodes;
    }
}
