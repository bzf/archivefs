use std::path::Path;

enum ArchiveEntry {}

#[test]
fn correct_path_removes_trailing_slashes() {
    let broken = String::from("/usr/local/bin/");
    let result = String::from("/usr/local/bin");
    assert_eq!(correct_path(broken), result);
}

#[test]
fn correct_path_handles_multiple_trailing_slashes() {
    let broken = String::from("/usr/local/bin///");
    let result = String::from("/usr/local/bin");
    assert_eq!(correct_path(broken), result);
}

pub fn correct_path(mut path: String) -> String {
    if path.ends_with("/") {
        let length = path.len();
        path.truncate(length - 1);
        return correct_path(path);
    } else {
        return path;
    }
}

#[test]
fn filename_without_rar_extension_works() {
    let filename = String::from("foobar.rar");
    let extension = String::from(".rar");
    assert_eq!(
        filename_without_extension(filename, &extension),
        String::from("foobar")
    );

    let filename = String::from("foobar.zip");
    assert_eq!(
        filename_without_extension(filename, &extension),
        String::from("foobar.zip")
    );
}

pub fn filename_without_extension(filename: String, extension: &str) -> String {
    return filename.replace(extension, "");
}

pub fn is_multipart_rar_file(path: String) -> bool {
    if !path.ends_with(".rar") {
        return false;
    }

    let mut filename = filename_without_extension(path, ".rar");
    filename.push_str(".r01");

    return Path::new(&filename).exists();
}

pub fn does_file_exist(path: &str) -> bool {
    return Path::new(path).exists();
}
