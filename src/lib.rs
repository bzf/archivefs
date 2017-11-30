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
