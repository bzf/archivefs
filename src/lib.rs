extern crate libc;
extern crate walkdir;

mod utils;
mod node;
mod archive;
mod ffi;
mod directory_archive;

use std::boxed::Box;
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::rc::Rc;
use node::Node;
use directory_archive::DirectoryArchive;

#[no_mangle]
pub extern "C" fn archivefs_node_name(node: *mut Node) -> *mut libc::c_char {
    let name: &str = unsafe { &(*node).name };

    let name: String = String::from(name);
    let c_result: CString = unsafe { CString::from_vec_unchecked(name.into_bytes()) };

    return c_result.into_raw();
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
pub extern "C" fn archivefs_directory_archive_get_node_in_directory(
    archive: *mut DirectoryArchive,
    prefix: *mut c_char,
    index: i64,
) -> *const Node {
    let prefix = unsafe { CStr::from_ptr(prefix) };
    let prefix: String = String::from(prefix.to_str().unwrap());

    let nodes: Vec<Rc<Node>> = unsafe { (*archive).get_nodes_in_directory(&prefix) };
    let node = nodes.get(index as usize);

    if let Some(node) = node {
        let n = node.clone();
        let ptr = Rc::into_raw(n);
        return ptr;
    } else {
        return ptr::null();
    }
}

#[no_mangle]
pub extern "C" fn archivefs_directory_archive_count_nodes_in_root(
    archive: *mut DirectoryArchive,
) -> i64 {
    let nodes: Vec<Rc<Node>> = unsafe { (*archive).list_files_in_root() };
    return nodes.len() as i64;
}

#[no_mangle]
pub extern "C" fn archivefs_directory_archive_get_node_in_root(
    archive: *mut DirectoryArchive,
    index: i64,
) -> *const Node {
    let nodes: Vec<Rc<Node>> = unsafe { (*archive).list_files_in_root() };
    let node = nodes.get(index as usize);

    if let Some(node) = node {
        return Rc::into_raw(node.clone());
    } else {
        panic!("archivefs_directory_archive_get_node_in_root: out of range");
    }
}


#[no_mangle]
pub extern "C" fn archivefs_directory_archive_count_nodes_in_directory(
    archive: *mut DirectoryArchive,
    prefix: *mut c_char,
) -> i64 {
    let prefix = unsafe { CStr::from_ptr(prefix) };
    let prefix: String = String::from(prefix.to_str().unwrap());

    let nodes: Vec<Rc<Node>> = unsafe { (*archive).get_nodes_in_directory(&prefix) };
    return nodes.len() as i64;
}


#[no_mangle]
pub extern "C" fn archivefs_directory_archive_new(raw_path: *mut c_char) -> *mut DirectoryArchive {
    let path = unsafe { CStr::from_ptr(raw_path) };
    let path: String = String::from(path.to_str().unwrap());

    let directory_archive: DirectoryArchive = DirectoryArchive::new(&path);
    let directory_archive_box = Box::new(directory_archive);
    let ptr: *mut DirectoryArchive = Box::into_raw(directory_archive_box);

    return ptr;
}

#[no_mangle]
pub extern "C" fn archivefs_directory_archive_get_node_for_path(
    archive: *mut DirectoryArchive,
    path: *mut c_char,
) -> *const Node {
    let path = unsafe { CStr::from_ptr(path) };
    let path: String = String::from(path.to_str().unwrap());

    let node: Option<Rc<Node>> = unsafe { (*archive).get_node_for_path(&path) };

    match node {
        Some(node) => {
            let ptr = Rc::into_raw(node);
            return ptr;
        }

        None => {
            return ptr::null();
        }
    }
}
