use std::path::Path;

#[test]
fn filename_without_rar_extension_works() {
    let filename = String::from("foobar.rar");
    let extension = String::from(".rar");
    assert_eq!(
        filename_without_extension(&filename, &extension),
        String::from("foobar")
    );

    let filename = String::from("foobar.zip");
    assert_eq!(
        filename_without_extension(&filename, &extension),
        String::from("foobar.zip")
    );
}

pub fn filename_without_extension(filename: &String, extension: &str) -> String {
    return filename.clone().replace(extension, "");
}

pub fn get_all_archive_filenames(path: &str) -> Vec<String> {
    let first_archive_path: String = String::from(path);

    let mut parts: Vec<String> = vec![first_archive_path.clone()];

    if is_multipart_rar_file(&first_archive_path) {
        let filename = filename_without_extension(&first_archive_path, ".rar");

        let mut rar_part_index: u32 = 0;
        loop {
            let filename_index = format!("{:02}", rar_part_index);
            let filename_part = format!("{}.r{}", filename, filename_index);

            if does_file_exist(&filename_part) {
                parts.push(filename_part);
            } else {
                break;
            }

            rar_part_index += 1;
        }
    }

    parts
}

pub fn is_multipart_rar_file(path: &String) -> bool {
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
