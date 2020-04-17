use filesystem_node::FilesystemNode;
use readable::Readable;

pub trait Browseable {
    fn name(&self) -> &str;

    fn clone(&self) -> Box<dyn Browseable>;

    fn list_subdirectories(&self) -> Vec<Box<dyn Browseable>>;

    fn list_files(&self) -> Vec<Box<dyn Readable>>;

    fn get_node(&self, path: &str) -> Option<FilesystemNode>;

    fn size(&self) -> u64 {
        4096
    }

    fn get_file(&self, filename: &str) -> Option<Box<dyn Readable>> {
        for file in self.list_files() {
            if file.filename() == filename {
                return Some(file.clone());
            }
        }

        None
    }

    fn get_subdirectory(&self, name: &str) -> Option<Box<dyn Browseable>> {
        let subdirectories = self.list_subdirectories();

        for subdirectory in subdirectories {
            if subdirectory.name() == name {
                return Some(subdirectory.clone());
            }
        }

        None
    }

    fn list_nodes(&self) -> Vec<FilesystemNode> {
        let mut nodes: Vec<FilesystemNode> = vec![];

        for file in self.list_files() {
            nodes.push(FilesystemNode::Readable(file));
        }

        for directory in self.list_subdirectories() {
            nodes.push(FilesystemNode::Browseable(directory));
        }

        nodes
    }
}
