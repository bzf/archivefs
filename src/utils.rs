use std::path::Path;

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
