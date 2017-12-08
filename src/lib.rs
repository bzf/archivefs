extern crate libc;

mod utils;
mod node;
mod archive;
mod ffi;

use std::boxed::Box;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use node::Node;
use archive::Archive;

#[no_mangle]
pub extern "C" fn archivefs_correct_path(raw_path: *mut c_char) -> *mut c_char {
    let path: CString = unsafe { CString::from_raw(raw_path) };

    let path: String = path.into_string().unwrap();
    let result = utils::correct_path(path);
    let c_result: CString = unsafe { CString::from_vec_unchecked(result.into_bytes()) };

    return c_result.into_raw();
}

#[no_mangle]
pub extern "C" fn archivefs_filename_without_extension(
    filename: *mut c_char,
    extension: *mut c_char,
) -> *mut c_char {
    let c_filename: CString = unsafe { CString::from_raw(filename) };
    let c_extension: CString = unsafe { CString::from_raw(extension) };

    let filename: String = c_filename.clone().into_string().unwrap();
    let extension: String = c_extension.clone().into_string().unwrap();
    let filename_without_extension: String =
        utils::filename_without_extension(filename, &extension);
    let c_result: CString =
        unsafe { CString::from_vec_unchecked(filename_without_extension.into_bytes()) };

    // Release ownership of the variables
    c_filename.into_raw();
    c_extension.into_raw();

    return c_result.into_raw();
}

#[no_mangle]
pub extern "C" fn archivefs_is_multipart_rar_file(path: *mut c_char) -> bool {
    let c_path: CString = unsafe { CString::from_raw(path) };

    let path: String = c_path.clone().into_string().unwrap();
    let result = utils::is_multipart_rar_file(path);

    // Release the path pointer
    c_path.into_raw();

    return result;
}

#[no_mangle]
pub extern "C" fn archivefs_new_node(
    path: *mut c_char,
    entry: *mut c_void,
    name: *mut c_char,
    buffer_size: libc::size_t,
) -> *mut Node {
    let path = unsafe { CStr::from_ptr(path) };
    let path: String = String::from(path.to_str().unwrap());

    let name = unsafe { CStr::from_ptr(name) };
    let name: String = String::from(name.to_str().unwrap());

    let node: Node = Node::new(
        path.clone(),
        entry as *mut ffi::ArchiveEntry,
        name.clone(),
        buffer_size,
    );

    let node_box = Box::new(node);
    let ptr: *mut node::Node = Box::into_raw(node_box);

    return ptr;
}

#[no_mangle]
pub extern "C" fn archivefs_node_is_directory(node: *mut Node) -> bool {
    if node.is_null() {
        return true;
    } else {
        return unsafe { (*node).is_directory() };
    }
}

#[no_mangle]
pub extern "C" fn archivefs_node_size(node: *mut Node) -> libc::int64_t {
    return unsafe { (*node).size() };
}

#[no_mangle]
pub fn archivefs_does_file_exist(ptr: *const c_char) -> bool {
    let path = unsafe { CStr::from_ptr(ptr) };

    let path: &str = path.to_str().unwrap();
    let result: bool = utils::does_file_exist(&path);

    return result;
}

#[no_mangle]
pub extern "C" fn archivefs_node_open(node: *mut Node) {
    return unsafe { (*node).open() };
}

#[no_mangle]
pub extern "C" fn archivefs_node_close(node: *mut Node) -> i64 {
    return unsafe { (*node).close() };
}

#[no_mangle]
pub extern "C" fn archivefs_node_write_to_buffer(
    node: *mut Node,
    buf: *mut c_char,
    size: libc::size_t,
    offset: libc::off_t,
) -> libc::size_t {
    return unsafe { (*node).write_to_buffer(buf, size, offset) };
}

#[no_mangle]
pub extern "C" fn archivefs_archive_new(raw_path: *mut c_char) -> *mut Archive {
    let path = unsafe { CStr::from_ptr(raw_path) };
    let path: String = String::from(path.to_str().unwrap());

    let archive: Archive = Archive::new(&path);
    let archive_box = Box::new(archive);
    let ptr: *mut Archive = Box::into_raw(archive_box);

    return ptr;
}
