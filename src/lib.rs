extern crate libc;
extern crate walkdir;

mod utils;
mod node;
mod archive;
mod ffi;
mod directory_archive;

use std::boxed::Box;
use std::ffi::{CStr, CString};
use std::os::raw::{c_void, c_char};
use std::ptr;
use std::rc::Rc;
use libc::{stat, off_t};
use node::Node;
use directory_archive::DirectoryArchive;

#[no_mangle]
pub fn archivefs_handle_getattr_callback(
    directory_archive: *mut DirectoryArchive,
    path: *mut c_char,
    stbuf: *mut stat,
) -> i32 {
    let path = unsafe { CStr::from_ptr(path) };
    let path: &str = path.to_str().unwrap();

    if let Some(node) = unsafe { (*directory_archive).get_node_for_path(&path) } {
        unsafe {
            (*stbuf).st_mode = if node.is_directory() {
                libc::S_IFDIR | 0o0777
            } else {
                libc::S_IFREG | 0o0777
            }
        };

        unsafe { (*stbuf).st_nlink = (node.is_directory() as u16) + 1 };
        if !node.is_directory() {
            unsafe { (*stbuf).st_size = node.size() };
        }

        return 0;
    }

    if path == "/" {
        unsafe { (*stbuf).st_mode = libc::S_IFDIR | 0o0755 };
        unsafe { (*stbuf).st_nlink = 2 };
        return 0;
    }

    return -libc::ENOENT;
}

#[no_mangle]
pub extern "C" fn archivefs_handle_readdir_callback(
    directory_archive: *mut DirectoryArchive,
    directory_prefix: *const c_char,
    buffer: *mut c_void,
    filler: extern "C" fn(*mut c_void, *const c_char, *const c_void, off_t) -> i32,
    _: off_t,
    _: *mut ffi::FuseFileInfo,
) -> i32 {
    let path = CString::new(".").unwrap();
    filler(buffer, path.as_ptr(), ptr::null(), 0);
    let path = CString::new("..").unwrap();
    filler(buffer, path.as_ptr(), ptr::null(), 0);

    let directory_prefix = unsafe { CStr::from_ptr(directory_prefix) };
    let directory_prefix: String = String::from(directory_prefix.to_str().unwrap());

    let nodes = if directory_prefix == "/" {
        unsafe { (*directory_archive).list_files_in_root() }
    } else {
        unsafe { (*directory_archive).get_nodes_in_directory(&directory_prefix) }
    };

    for node in nodes {
        let node_name: &str = &node.name;
        let node_name = CString::new(node_name).unwrap();

        let node_ptr = node_name.into_raw();
        filler(buffer, node_ptr, ptr::null(), 0);
        let _ = unsafe { CString::from_raw(node_ptr) };
    }

    return 0;
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
