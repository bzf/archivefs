mod fs_file;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

use regex::Regex;

use browseable::Browseable;
use ffi;
use filesystem_node::FilesystemNode;
use fs_archive::fs_file::FSFile;
use readable::Readable;

pub struct FSArchive {
    archive_path: String,
    name: String,
}

impl FSArchive {
    pub fn new(archive_path: &str) -> FSArchive {
        let path = std::path::Path::new(archive_path);

        let regex = Regex::new(r"(\.tar$)").unwrap();
        let name = regex.replace_all(path.file_stem().unwrap().to_str().unwrap(), "");

        FSArchive {
            archive_path: String::from(archive_path),
            name: String::from(name),
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
}

impl Browseable for FSArchive {
    fn name(&self) -> &str {
        &self.name
    }

    fn clone(&self) -> Box<dyn Browseable> {
        Box::new(FSArchive::new(&self.archive_path))
    }

    fn list_subdirectories(&self) -> Vec<Box<dyn Browseable>> {
        vec![]
    }

    fn list_files(&self) -> Vec<Box<dyn Readable>> {
        let mut archive_files: Vec<Box<dyn Readable>> = vec![];
        let archive: *mut ffi::archive = unsafe { ffi::archive_read_new() };

        unsafe {
            ffi::archive_read_support_filter_all(archive);
            ffi::archive_read_support_format_all(archive);
            ffi::archive_open_and_read_from_path(&self.archive_path, archive, 8192);
        };

        let mut archive_entry: *mut ffi::archive_entry = ptr::null_mut();

        loop {
            if unsafe { ffi::archive_read_next_header(archive, &mut archive_entry) != 0 } {
                break;
            }

            let archive_pathname: *const c_char =
                unsafe { ffi::archive_entry_pathname(archive_entry) };
            let archive_path = unsafe { CStr::from_ptr(archive_pathname) };
            let archive_pathname: String = String::from(archive_path.to_str().unwrap());

            let filesize: u64 = unsafe { ffi::archive_entry_size(archive_entry) } as u64;

            let archive_file: FSFile = FSFile::new(&self.archive_path, filesize, &archive_pathname);
            archive_files.push(Box::new(archive_file));

            unsafe { ffi::archive_read_data_skip(archive) };
        }

        unsafe { ffi::archive_read_free(archive) };

        archive_files
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
