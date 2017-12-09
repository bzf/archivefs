extern crate libc;

use libc::{size_t, off_t};
use std::os::raw::c_char;
use std::ffi::CStr;
use std::ptr;

use ffi;
use ffi::{Archive, ArchiveEntry};

pub struct Node {
    archive_path: String,
    entry: *mut ArchiveEntry,
    buffer_size: libc::size_t,
    pub archive: Option<*mut Archive>,
    pub name: String,
}

impl Node {
    pub fn new(
        archive_path: String,
        entry: *mut ArchiveEntry,
        name: String,
        buffer_size: libc::size_t,
    ) -> Node {
        let node: Node = Node {
            archive_path: archive_path,
            entry: entry,
            buffer_size: buffer_size,
            name: name,
            archive: None,
        };

        return node;
    }

    pub fn is_directory(&self) -> bool {
        if self.entry.is_null() {
            return true;
        } else {
            return unsafe { ffi::archive_entry_filetype(self.entry) == 16384 };
        }
    }

    pub fn size(&self) -> i64 {
        return unsafe { ffi::archive_entry_size(self.entry) };
    }

    pub fn open(&mut self) {
        if let Some(_) = self.archive {
            return;
        }

        let archive: *mut Archive = unsafe { ffi::archive_read_new() };
        unsafe { ffi::archive_read_support_filter_all(archive) };
        unsafe { ffi::archive_read_support_format_all(archive) };

        ffi::archive_open_and_read_from_path(&self.archive_path, archive, self.buffer_size);

        let mut entry: *mut ArchiveEntry = ptr::null_mut();
        let our_entry_path: *const c_char = unsafe { ffi::archive_entry_pathname(self.entry) };
        let our_entry_path = unsafe { CStr::from_ptr(our_entry_path) };
        let our_entry_path: &str = our_entry_path.to_str().unwrap();

        while unsafe { ffi::archive_read_next_header(archive, &mut entry) == 0x0 } {
            let their_entry_path: *const c_char = unsafe { ffi::archive_entry_pathname(entry) };
            let their_entry_path = unsafe { CStr::from_ptr(their_entry_path) };
            let their_entry_path: &str = their_entry_path.to_str().unwrap();

            if our_entry_path == their_entry_path {
                self.archive = Some(archive);
                return;
            }
        }

        panic!("Could not find the path in the archive");
    }

    pub fn write_to_buffer(&mut self, buffer: *mut c_char, size: size_t, offset: off_t) -> size_t {
        if let Some(archive) = self.archive {
            if offset != -1 {
                unsafe { ffi::archive_seek_data(archive, offset, 0) };
            }

            let bytes_written = unsafe { ffi::archive_read_data(archive, buffer, size) };
            return bytes_written as size_t;
        } else {
            panic!("Must open archive before writing from it");
        }
    }

    pub fn close(&mut self) -> i64 {
        if let Some(archive) = self.archive {
            let result = unsafe { ffi::archive_read_free(archive) };
            self.archive = None;
            return result;
        } else {
            return -1;
        }
    }
}
