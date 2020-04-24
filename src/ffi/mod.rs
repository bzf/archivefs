extern crate libc;

use libc::size_t;
use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

use utils;

pub enum ArchiveEntry {}
pub enum Archive {}

extern "C" {
    pub fn archive_read_data(_: *mut Archive, _: *mut c_char, _: size_t) -> size_t;
    pub fn archive_read_data_skip(_: *mut Archive) -> i64;

    pub fn archive_seek_data(_: *mut Archive, _: i64, _: i64) -> i64;

    pub fn archive_read_next_header(_: *mut Archive, _: *mut *mut ArchiveEntry) -> i64;

    pub fn archive_entry_pathname(_: *mut ArchiveEntry) -> *const c_char;
    pub fn _archive_entry_filetype(_: *mut ArchiveEntry) -> libc::mode_t;
    pub fn archive_entry_size(_: *mut ArchiveEntry) -> i64;

    pub fn archive_read_new() -> *mut Archive;
    pub fn archive_read_support_filter_all(_: *mut Archive) -> i64;
    pub fn archive_read_support_format_all(_: *mut Archive) -> i64;

    pub fn archive_read_open_filenames(
        archive: *mut Archive,
        filenames: *const *const c_char,
        block_size: libc::size_t,
    ) -> i64;

    pub fn archive_read_free(_: *mut Archive) -> i64;
}

pub struct FuseFileInfo {}

pub fn archive_open_and_read_from_path(
    path: &str,
    archive: *mut Archive,
    buffer_size: libc::size_t,
) -> i64 {
    let parts: Vec<String> = utils::get_all_archive_filenames(&path);

    let cparts: Vec<CString> = parts
        .iter()
        .map(|x| CString::new(x.as_str()).unwrap())
        .collect();

    let mut ptr_parts: Vec<*const c_char> = cparts.iter().map(|x| x.as_ptr()).collect();
    ptr_parts.push(ptr::null_mut());
    let vec: *const *const c_char = ptr_parts.as_ptr();

    unsafe {
        return archive_read_open_filenames(archive, vec, buffer_size);
    };
}
