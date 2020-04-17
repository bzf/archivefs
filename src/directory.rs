use file::File;

#[derive(Debug)]
pub struct Directory {
    dirpath: String,
}

impl Directory {
    pub fn new(dirpath: &str) -> Directory {
        Directory {
            dirpath: String::from(dirpath),
        }
    }

    pub fn clone(&self) -> Directory {
        Directory::new(&self.dirpath)
    }

    pub fn name(&self) -> &str {
        self.path().file_stem().unwrap().to_str().unwrap()
    }

    pub fn size(&self) -> u64 {
        4096
    }

    pub fn get_file(&self, filename: &str) -> Option<File> {
        let files = self.list_files();

        match files.iter().find(|file| file.filename() == filename) {
            None => None,
            Some(file) => Some(file.clone()),
        }
    }

    pub fn list_files(&self) -> Vec<File> {
        let entries = std::fs::read_dir(&self.dirpath).unwrap();

        entries
            .filter(|e| !e.as_ref().unwrap().path().is_dir())
            .map(|e| File::new(&e.unwrap().path().to_str().unwrap()))
            .collect()
    }

    pub fn list_subdirectories(&self) -> Vec<Directory> {
        let entries = std::fs::read_dir(&self.dirpath).unwrap();

        entries
            .filter(|e| e.as_ref().unwrap().path().is_dir())
            .map(|e| Directory::new(&e.unwrap().path().to_str().unwrap()))
            .collect()
    }

    pub fn get_subdirectory(&self, name: &str) -> Option<Directory> {
        let subdirectories = self.list_subdirectories();
        let subdirectory: Option<&Directory> = subdirectories.iter().find(|dir| dir.name() == name);

        match subdirectory {
            None => None,
            Some(dir) => Some(dir.clone()),
        }
    }

    fn path(&self) -> &std::path::Path {
        std::path::Path::new(&self.dirpath)
    }
}
