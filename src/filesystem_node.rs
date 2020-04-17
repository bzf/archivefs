use directory::Directory;
use file::File;

pub enum FilesystemNode {
    File(File),
    Directory(Directory),
}
