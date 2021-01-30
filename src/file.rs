use std::io::{Read, Seek};

use libc::{off_t, size_t};

use readable::Readable;

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

    fn path(&self) -> &std::path::Path {
        std::path::Path::new(&self.filepath)
    }
}

impl Readable for File {
    fn clone(&self) -> Box<dyn Readable> {
        Box::new(File::new(&self.filepath))
    }

    fn filename(&self) -> &str {
        self.path().file_name().unwrap().to_str().unwrap()
    }

    fn size(&self) -> u64 {
        let metadata = std::fs::metadata(&self.filepath).unwrap();
        metadata.len()
    }

    fn write_to_buffer(&self, buffer_ptr: *mut [u8], _size: size_t, offset: off_t) -> size_t {
        let mut file = std::fs::File::open(&self.filepath).unwrap();
        let mut buffer: &mut [u8] = unsafe { &mut *buffer_ptr };
        file.seek(std::io::SeekFrom::Start(offset as u64)).unwrap();
        file.read(&mut buffer).unwrap()
    }
}
