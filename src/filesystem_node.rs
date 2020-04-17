use browseable::Browseable;
use readable::Readable;

pub enum FilesystemNode {
    Readable(Box<dyn Readable>),
    Browseable(Box<dyn Browseable>),
}
