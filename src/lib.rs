use std::ffi::CString;
use std::os::raw::c_char;

mod utils;

#[no_mangle]
pub extern "C" fn archivefs_correct_path(raw_path: *mut c_char) -> *mut c_char {
    let path: CString = unsafe { CString::from_raw(raw_path) };

    let path: String = path.into_string().unwrap();
    let result = utils::correct_path(path);
    let c_result: CString = unsafe { CString::from_vec_unchecked(result.into_bytes()) };

    return c_result.into_raw();
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
    let filename_without_extension: String =
        utils::filename_without_extension(filename, &extension);
    let c_result: CString =
        unsafe { CString::from_vec_unchecked(filename_without_extension.into_bytes()) };

    // Release ownership of the variables
    c_filename.into_raw();
    c_extension.into_raw();

    return c_result.into_raw();
}
