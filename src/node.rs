extern crate libc;

use std::os::raw::c_void;

enum ArchiveEntry {}

#[link(name = "archive")]
extern "C" {
    fn archive_entry_filetype(_: *mut ArchiveEntry) -> libc::mode_t;
}

pub struct Node {
    path: String,
    entry: *mut ArchiveEntry,
    name: String,
    buffer_size: libc::size_t,
}

impl Node {
    pub fn new(path: String, entry: *mut c_void, name: String, buffer_size: libc::size_t) -> Node {
        let node: Node = Node {
            path: path,
            entry: entry as *mut ArchiveEntry,
            name: name,
            buffer_size: buffer_size,
        };

        return node;
    }

    pub fn is_directory(&self) -> bool {
        if self.entry.is_null() {
            return true;
        } else {
            return unsafe { archive_entry_filetype(self.entry) == 16384 };
        }
    }
}
