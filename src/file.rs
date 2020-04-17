use std::io::Write;
use tempdir::TempDir;

pub struct File {
    filepath: String,
}

impl File {
    pub fn new(filepath: &str) -> File {
        File {
            filepath: String::from(filepath),
        }
    }

    pub fn clone(&self) -> File {
        File::new(&self.filepath)
    }

    pub fn filename(&self) -> &str {
        self.path().file_name().unwrap().to_str().unwrap()
    }

    pub fn size(&self) -> u64 {
        let metadata = std::fs::metadata(&self.filepath).unwrap();
        metadata.len()
    }

    fn path(&self) -> &std::path::Path {
        std::path::Path::new(&self.filepath)
    }
}

#[test]
fn test_file_size() {
    let tmp_dir = TempDir::new("example").unwrap();

    let file_path = tmp_dir.path().join("my-temporary-note.txt");
    let mut tmp_file = std::fs::File::create(&file_path).unwrap();

    let content = "Brian was here. Briefly.";
    writeln!(tmp_file, "{}", content).unwrap();

    println!("{:?}", file_path);
    let file = File::new(file_path.as_path().to_str().unwrap());
    assert_eq!((content.len() + 1) as u64, file.size());
}

#[test]
fn test_file_name() {
    let tmp_dir = TempDir::new("example").unwrap();

    let file_path = tmp_dir.path().join("my-temporary-note.txt");
    let mut tmp_file = std::fs::File::create(&file_path).unwrap();

    writeln!(tmp_file, "Hello").unwrap();

    println!("{:?}", file_path);
    let file = File::new(file_path.as_path().to_str().unwrap());
    assert_eq!(file.filename(), "my-temporary-note.txt");
}
