use std::io::{Read, Seek, Write};
use tempdir::TempDir;

use libc::{off_t, size_t};
use std::os::raw::c_char;

#[derive(Debug)]
pub struct File {
    filepath: String,
}

impl File {
    pub fn new(filepath: &str) -> File {
        File {
            filepath: String::from(filepath),
        }
    }

    pub fn clone(&self) -> File {
        File::new(&self.filepath)
    }

    pub fn filename(&self) -> &str {
        self.path().file_name().unwrap().to_str().unwrap()
    }

    pub fn size(&self) -> u64 {
        let metadata = std::fs::metadata(&self.filepath).unwrap();
        metadata.len()
    }

    pub fn write_to_buffer(&self, buffer_ptr: *mut [u8], size: size_t, offset: off_t) -> size_t {
        let mut file = std::fs::File::open(&self.filepath).unwrap();
        let mut buffer: &mut [u8] = unsafe { &mut *buffer_ptr };
        file.seek(std::io::SeekFrom::Start(offset as u64)).unwrap();
        file.read(&mut buffer).unwrap()
    }

    fn path(&self) -> &std::path::Path {
        std::path::Path::new(&self.filepath)
    }
}

#[test]
fn test_file_size() {
    let tmp_dir = TempDir::new("example").unwrap();

    let file_path = tmp_dir.path().join("my-temporary-note.txt");
    let mut tmp_file = std::fs::File::create(&file_path).unwrap();

    let content = "Brian was here. Briefly.";
    writeln!(tmp_file, "{}", content).unwrap();

    let file = File::new(file_path.as_path().to_str().unwrap());
    assert_eq!((content.len() + 1) as u64, file.size());
}

#[test]
fn test_file_name() {
    let tmp_dir = TempDir::new("example").unwrap();

    let file_path = tmp_dir.path().join("my-temporary-note.txt");
    let mut tmp_file = std::fs::File::create(&file_path).unwrap();

    writeln!(tmp_file, "Hello").unwrap();

    let file = File::new(file_path.as_path().to_str().unwrap());
    assert_eq!(file.filename(), "my-temporary-note.txt");
}
