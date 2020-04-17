use directory::Directory;
use filesystem_node::FilesystemNode;
use readable::Readable;

pub trait Browseable {
    fn name(&self) -> &str;

    fn size(&self) -> u64 {
        4096
    }

    fn get_file(&self, filename: &str) -> Option<Box<dyn Readable>>;

    fn get_subdirectory(&self, name: &str) -> Option<Directory>;

    fn get_node(&self, path: &str) -> Option<FilesystemNode>;

    fn list_nodes(&self) -> Vec<FilesystemNode>;
}
