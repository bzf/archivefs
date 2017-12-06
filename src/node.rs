extern crate libc;

use libc::{size_t, off_t};
use std::os::raw::{c_char, c_void};
use std::ffi::{CStr, CString};
use std::vec::Vec;
use std::{ptr, mem};
use utils;

enum ArchiveEntry {}
pub enum Archive {}

#[link(name = "archive")]
extern "C" {
    // __LA_DECL la_ssize_t		 archive_read_data(struct archive *,
    // 				    void *, size_t);
    fn archive_read_data(_: *mut Archive, _: *mut c_char, _: size_t) -> size_t;

    // __LA_DECL la_int64_t archive_seek_data(struct archive *, la_int64_t, int);
    fn archive_seek_data(_: *mut Archive, _: i64, _: i64) -> i64;

    // __LA_DECL int archive_read_next_header(struct archive *,
    // 		     struct archive_entry **);
    fn archive_read_next_header(_: *mut Archive, _: *mut (*mut ArchiveEntry)) -> i64;

    // __LA_DECL const char	*archive_entry_pathname(struct archive_entry *);
    fn archive_entry_pathname(_: *mut ArchiveEntry) -> *const c_char;
    fn archive_entry_filetype(_: *mut ArchiveEntry) -> libc::mode_t;
    fn archive_entry_size(_: *mut ArchiveEntry) -> i64;
    fn archive_read_new() -> *mut Archive;
    fn archive_read_support_filter_all(_: *mut Archive) -> i64;
    fn archive_read_support_format_all(_: *mut Archive) -> i64;

    // __LA_DECL int archive_read_open_filename(struct archive *,
    //      const char *_filename, size_t _block_size);
    fn archive_read_open_filename(
        archive: *mut Archive,
        filename: *mut c_char,
        block_size: libc::size_t,
    ) -> i64;

    // __LA_DECL int archive_read_open_filename(struct archive *,
    // const char *_filename, size_t _block_size);
    fn archive_read_open_filenames(
        archive: *mut Archive,
        filenames: *mut (*mut c_char),
        block_size: libc::size_t,
    ) -> i64;
}

pub struct Node {
    path: String,
    entry: *mut ArchiveEntry,
    name: String,
    buffer_size: libc::size_t,
    pub archive: Option<*mut Archive>,
}

impl Node {
    pub fn new(path: String, entry: *mut c_void, name: String, buffer_size: libc::size_t) -> Node {
        let node: Node = Node {
            path: path,
            entry: entry as *mut ArchiveEntry,
            name: name,
            buffer_size: buffer_size,
            archive: None,
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

    pub fn size(&self) -> i64 {
        return unsafe { archive_entry_size(self.entry) };
    }

    pub fn open(&mut self) {
        if let Some(_) = self.archive {
            return;
        }

        /* if (_archive != nullptr) { */
        /*     return; */
        /* } */

        let archive: *mut Archive = unsafe { archive_read_new() };
        unsafe { archive_read_support_filter_all(archive) };
        unsafe { archive_read_support_format_all(archive) };

        archive_open_and_read_from_path(&self.path, archive, self.buffer_size);

        let mut entry: *mut ArchiveEntry = ptr::null_mut();
        let our_entry_path: *const c_char = unsafe { archive_entry_pathname(self.entry) };
        let our_entry_path = unsafe { CStr::from_ptr(our_entry_path) };
        let our_entry_path: &str = our_entry_path.to_str().unwrap();

        while unsafe { archive_read_next_header(archive, &mut entry) == 0x0 } {
            let their_entry_path: *const c_char = unsafe { archive_entry_pathname(entry) };
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
                unsafe { archive_seek_data(archive, offset, 0) };
            }

            let bytes_written = unsafe { archive_read_data(archive, buffer, size) };
            return bytes_written as size_t;
        } else {
            panic!("Must open archive before writing from it");
        }
    }
}

fn archive_open_and_read_from_path(
    path: &str,
    archive: *mut Archive,
    buffer_size: libc::size_t,
) -> i64 {
    let path: String = String::from(path);

    if utils::is_multipart_rar_file(path.clone()) {
        let filename = utils::filename_without_extension(path.clone(), ".rar");
        let mut parts: Vec<String> = vec![path]; // Vec::new();

        let mut rar_part_index: u32 = 0;
        loop {
            let filename_index = format!("{:02}", rar_part_index);
            let filename_part = format!("{}.r{}", filename, filename_index);

            if utils::does_file_exist(&filename_part) {
                parts.push(filename_part);
            } else {
                break;
            }

            rar_part_index += 1;
        }

        let parts: Vec<CString> = parts
            .into_iter()
            .map(|x| CString::new(x).unwrap())
            .collect();
        let mut parts: Vec<*mut c_char> = parts.into_iter().map(|x| x.into_raw()).collect();
        parts.push(ptr::null_mut());

        parts.shrink_to_fit();
        let vec: *mut (*mut c_char) = parts.as_mut_ptr();

        mem::forget(vec); // prevent deallocation in Rust
        // The array is still there but no Rust object
        // feels responsible. We only have ptr/len now
        // to reach it.

        unsafe {
            return archive_read_open_filenames(archive, vec, buffer_size);
        }
    } else {
        let path_bytes = path.clone().into_bytes();
        let path_ptr = CString::new(path_bytes).unwrap().into_raw(); // = CString

        unsafe {
            return archive_read_open_filename(archive, path_ptr, buffer_size);
        };
    }
}
