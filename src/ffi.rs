extern crate libc;

use libc::size_t;
use std::ffi::CString;
use std::os::raw::c_char;
use std::{ptr, mem};

use utils;

pub enum ArchiveEntry {}
pub enum Archive {}

extern "C" {
    pub fn archive_read_data(_: *mut Archive, _: *mut c_char, _: size_t) -> size_t;
    pub fn archive_read_data_skip(_: *mut Archive) -> i64;

    pub fn archive_seek_data(_: *mut Archive, _: i64, _: i64) -> i64;

    pub fn archive_read_next_header(_: *mut Archive, _: *mut (*mut ArchiveEntry)) -> i64;

    pub fn archive_entry_pathname(_: *mut ArchiveEntry) -> *const c_char;
    pub fn archive_entry_filetype(_: *mut ArchiveEntry) -> libc::mode_t;
    pub fn archive_entry_size(_: *mut ArchiveEntry) -> i64;

    pub fn archive_read_new() -> *mut Archive;
    pub fn archive_read_support_filter_all(_: *mut Archive) -> i64;
    pub fn archive_read_support_format_all(_: *mut Archive) -> i64;

    pub fn archive_read_open_filename(
        archive: *mut Archive,
        filename: *mut c_char,
        block_size: libc::size_t,
    ) -> i64;

    pub fn archive_read_open_filenames(
        archive: *mut Archive,
        filenames: *mut (*mut c_char),
        block_size: libc::size_t,
    ) -> i64;

    pub fn archive_read_free(_: *mut Archive) -> i64;
}

pub fn archive_open_and_read_from_path(
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
