use browseable::Browseable;
use directory::Directory;
use filesystem_node::FilesystemNode;
use readable::Readable;

#[derive(Debug)]
pub struct Filesystem {
    path: String,
}

impl Filesystem {
    pub fn new(path: &str) -> Filesystem {
        Filesystem {
            path: String::from(path),
        }
    }

    pub fn list_files(&self, path: &str) -> Vec<Box<dyn Readable>> {
        if path == "/" {
            self.root_directory().list_files()
        } else {
            match self.get_directory(&path) {
                Some(dir) => dir.list_files(),
                None => vec![],
            }
        }
    }

    pub fn get_directory(&self, path: &str) -> Option<Box<dyn Browseable>> {
        if path == "/" {
            return Some(self.root_directory());
        }

        let fragments: Vec<&str> = path.split('/').filter(|x| x != &"").collect();

        let mut current_directory = self.root_directory();

        for directory in fragments {
            match current_directory.get_subdirectory(directory) {
                None => return None,
                Some(dir) => current_directory = dir,
            }
        }

        Some(current_directory)
    }

    pub fn get_node(&self, path: &str) -> Option<FilesystemNode> {
        if path == "/" {
            Some(FilesystemNode::Browseable(self.root_directory().clone()))
        } else {
            self.root_directory().get_node(path)
        }
    }

    fn root_directory(&self) -> Box<dyn Browseable> {
        Box::new(Directory::new(&self.path))
    }
}
