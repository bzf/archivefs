extern crate libc;
extern crate regex;
extern crate tempdir;
extern crate walkdir;

mod archive;
mod browseable;
mod directory;
mod directory_archive;
mod ffi;
mod file;
mod filesystem;
mod filesystem_node;
mod fs_archive;
mod node;
mod readable;
mod utils;

use directory_archive::DirectoryArchive;
use ffi::FuseFileInfo;
use filesystem::Filesystem;
use filesystem_node::FilesystemNode;
use libc::{off_t, stat};
use std::boxed::Box;
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_void};
use std::ptr;

#[no_mangle]
pub fn archivefs_handle_getattr_callback(
    filesystem_ptr: *mut Filesystem,
    path: *mut c_char,
    stbuf: *mut stat,
) -> i32 {
    let path = unsafe { CStr::from_ptr(path) };
    let path: &str = path.to_str().unwrap();
    let filesystem: &Filesystem = unsafe { &*filesystem_ptr };

    if let Some(fs_node) = filesystem.get_node(path) {
        match fs_node {
            FilesystemNode::Readable(file) => unsafe {
                (*stbuf).st_mode = libc::S_IFREG | 0o0777;
                (*stbuf).st_nlink = 2;
                (*stbuf).st_size = file.size() as i64;
            },

            FilesystemNode::Browseable(dir) => unsafe {
                (*stbuf).st_mode = libc::S_IFDIR | 0o0777;
                (*stbuf).st_nlink = 1;
                (*stbuf).st_size = dir.size() as i64;
            },
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
    filesystem_ptr: *mut Filesystem,
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

    let filesystem: &Filesystem = unsafe { &*filesystem_ptr };

    let directory_prefix = unsafe { CStr::from_ptr(directory_prefix) };
    let directory_prefix: String = String::from(directory_prefix.to_str().unwrap());

    if let Some(directory) = filesystem.get_directory(&directory_prefix) {
        for node in directory.list_nodes() {
            match node {
                FilesystemNode::Readable(file) => {
                    let node_name: &str = &file.filename();
                    let node_name = CString::new(node_name).unwrap();

                    let node_ptr = node_name.into_raw();
                    filler(buffer, node_ptr, ptr::null(), 0);
                    let _ = unsafe { CString::from_raw(node_ptr) };
                }
                FilesystemNode::Browseable(dir) => {
                    let node_name: &str = &dir.name();
                    let node_name = CString::new(node_name).unwrap();

                    let node_ptr = node_name.into_raw();
                    filler(buffer, node_ptr, ptr::null(), 0);
                    let _ = unsafe { CString::from_raw(node_ptr) };
                }
            }
        }
    }

    return 0;
}

#[no_mangle]
pub extern "C" fn archivefs_handle_read_callback(
    filesystem_ptr: *mut Filesystem,
    path: *const c_char,
    buffer: *mut c_char,
    size: libc::size_t,
    offset: libc::off_t,
    _file_info: *mut FuseFileInfo,
) -> i32 {
    let path = unsafe { CStr::from_ptr(path) };
    let path: String = String::from(path.to_str().unwrap());

    let filesystem: &Filesystem = unsafe { &*filesystem_ptr };

    match filesystem.get_node(&path) {
        Some(FilesystemNode::Readable(file)) => {
            let buf_slice: *mut [u8] =
                unsafe { std::slice::from_raw_parts_mut(buffer as *mut u8, size) };
            file.write_to_buffer(buf_slice, size, offset) as i32
        }
        _ => {
            return -libc::ENOENT;
        }
    }
}

#[no_mangle]
pub extern "C" fn archivefs_handle_release_callback(
    filesystem_ptr: *mut Filesystem,
    _path: *const c_char,
    _file_info: *mut FuseFileInfo,
) -> i32 {
    let _filesystem: &Filesystem = unsafe { &*filesystem_ptr };

    return 0;
}

#[no_mangle]
pub extern "C" fn archivefs_filesystem_new(raw_path: *mut c_char) -> *mut Filesystem {
    let path = unsafe { CStr::from_ptr(raw_path) };
    let path: String = String::from(path.to_str().unwrap());

    let filesystem: Filesystem = Filesystem::new(&path);
    let filesystem_box = Box::new(filesystem);
    let ptr: *mut Filesystem = Box::into_raw(filesystem_box);

    return ptr;
}
