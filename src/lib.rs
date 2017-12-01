use std::ffi::CString;
use std::os::raw::c_char;

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

fn correct_path(mut path: String) -> String {
    if path.ends_with("/") {
        let length = path.len();
        path.truncate(length - 1);
        return correct_path(path);
    } else {
        return path;
    }
}

#[no_mangle]
pub extern "C" fn archivefs_correct_path(raw_path: *mut c_char) -> *mut c_char {
    let path: CString = unsafe { CString::from_raw(raw_path) };

    let path: String = path.into_string().unwrap();
    let result = correct_path(path);
    let c_result: CString = unsafe { CString::from_vec_unchecked(result.into_bytes()) };

    return c_result.into_raw();
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

fn filename_without_extension(filename: String, extension: &str) -> String {
    return filename.replace(extension, "");
}

#[no_mangle]
pub extern "C" fn archivefs_filename_without_extension(
    filename: *mut c_char,
    extension: *mut c_char,
) -> *mut c_char {
    let c_filename: CString = unsafe { CString::from_raw(filename) };
    let c_extension: CString = unsafe { CString::from_raw(extension) };

    let filename: String = c_filename.clone().into_string().unwrap();
    let extension: String = c_extension.clone().into_string().unwrap();
    let filename_without_extension: String = filename_without_extension(filename, &extension);
    let c_result: CString =
        unsafe { CString::from_vec_unchecked(filename_without_extension.into_bytes()) };

    // Release ownership of the variables
    c_filename.into_raw();
    c_extension.into_raw();

    return c_result.into_raw();
}
