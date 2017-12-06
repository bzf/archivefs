pub struct Archive {
    path: String,
}

impl Archive {
    pub fn new(path: &str) -> Archive {
        let path: String = String::from(path);
        let archive: Archive = Archive { path: path };

        return archive;
    }
}
