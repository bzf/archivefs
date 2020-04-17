use libc::{off_t, size_t};

pub trait Readable {
    fn filename(&self) -> &str;

    fn size(&self) -> u64;

    fn write_to_buffer(&self, buffer_ptr: *mut [u8], size: size_t, offset: off_t) -> size_t;
}
