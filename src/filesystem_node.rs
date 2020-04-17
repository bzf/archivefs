use directory::Directory;
use file::File;

#[derive(Debug)]
pub enum FilesystemNode {
    File(File),
    Directory(Directory),
}
