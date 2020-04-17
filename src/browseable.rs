pub trait Browseable {
    fn name(&self) -> &str;

    fn size(&self) -> u64 {
        4096
    }
}
