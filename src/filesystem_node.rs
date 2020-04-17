use directory::Directory;
use readable::Readable;

pub enum FilesystemNode {
    Readable(Box<dyn Readable>),
    Directory(Directory),
}
