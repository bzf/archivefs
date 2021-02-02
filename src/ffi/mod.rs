extern crate libc;

use std::ffi::CString;
use std::os::raw::c_char;
use std::ptr;

use utils;

pub use {
    archive, archive_entry, archive_entry_pathname, archive_entry_size, archive_read_data,
    archive_read_data_skip, archive_read_free, archive_read_new, archive_read_next_header,
    archive_read_open_filenames, archive_read_support_filter_all, archive_read_support_format_all,
    archive_seek_data,
};

pub struct FuseFileInfo {}

pub fn archive_open_and_read_from_path(
    path: &str,
    archive: *mut archive,
    buffer_size: libc::size_t,
) -> i64 {
    let parts: Vec<String> = utils::get_all_archive_filenames(&path);

    let cparts: Vec<CString> = parts
        .iter()
        .map(|x| CString::new(x.as_str()).unwrap())
        .collect();

    let mut ptr_parts: Vec<*const c_char> = cparts.iter().map(|x| x.as_ptr()).collect();
    ptr_parts.push(ptr::null_mut());
    let vec: *mut *const c_char = ptr_parts.as_ptr() as *mut *const c_char;

    unsafe {
        return archive_read_open_filenames(archive, vec, buffer_size as u64) as i64;
    };
}
