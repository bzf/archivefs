use libc::{off_t, size_t};

use std::ffi::CStr;
use std::os::raw::c_char;
use std::ptr;

use ffi;
use readable::Readable;

pub struct FSFile {
    archive_path: String,
    path: String,
    filesize: u64,
}

impl FSFile {
    pub fn new(path: &str, filesize: u64, archive_path: &str) -> FSFile {
        FSFile {
            path: String::from(path),
            filesize,
            archive_path: String::from(archive_path),
        }
    }

    fn open(&self) -> *mut ffi::archive {
        let archive: *mut ffi::archive = unsafe { ffi::archive_read_new() };
        unsafe { ffi::archive_read_support_filter_all(archive) };
        unsafe { ffi::archive_read_support_format_all(archive) };

        ffi::archive_open_and_read_from_path(&self.path, archive, 8192);

        let mut entry: *mut ffi::archive_entry = ptr::null_mut();
        let our_entry_path: &str = &self.archive_path;

        while unsafe { ffi::archive_read_next_header(archive, &mut entry) == 0x0 } {
            let their_entry_path: *const c_char = unsafe { ffi::archive_entry_pathname(entry) };
            let their_entry_path = unsafe { CStr::from_ptr(their_entry_path) };
            let their_entry_path: &str = their_entry_path.to_str().unwrap();

            if our_entry_path == their_entry_path {
                return archive;
            }
        }

        panic!("Could not find the path in the archive");
    }

    fn path(&self) -> &std::path::Path {
        std::path::Path::new(&self.archive_path)
    }
}

impl Readable for FSFile {
    fn clone(&self) -> Box<dyn Readable> {
        Box::new(FSFile::new(&self.path, self.filesize, &self.archive_path))
    }

    fn filename(&self) -> &str {
        self.path().file_name().unwrap().to_str().unwrap()
    }

    fn size(&self) -> u64 {
        self.filesize
    }

    fn write_to_buffer(&self, buffer_ptr: *mut [u8], size: size_t, offset: off_t) -> size_t {
        let archive_ptr: *mut ffi::archive = self.open();

        if offset != -1 {
            unsafe { ffi::archive_seek_data(archive_ptr, offset, 0) };
        }

        let bytes_written = unsafe {
            ffi::archive_read_data(archive_ptr, buffer_ptr as *mut libc::c_void, size as u64)
        };

        unsafe { ffi::archive_read_free(archive_ptr) };

        return bytes_written as size_t;
    }
}
