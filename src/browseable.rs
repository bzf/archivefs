use filesystem_node::FilesystemNode;
use readable::Readable;

pub trait Browseable {
    fn name(&self) -> &str;

    fn clone(&self) -> Box<dyn Browseable>;

    fn size(&self) -> u64 {
        4096
    }

    fn list_subdirectories(&self) -> Vec<Box<dyn Browseable>>;

    fn list_files(&self) -> Vec<Box<dyn Readable>>;

    fn get_file(&self, filename: &str) -> Option<Box<dyn Readable>>;

    fn get_subdirectory(&self, name: &str) -> Option<Box<dyn Browseable>>;

    fn get_node(&self, path: &str) -> Option<FilesystemNode>;

    fn list_nodes(&self) -> Vec<FilesystemNode>;
}
