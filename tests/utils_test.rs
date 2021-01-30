extern crate archivefs;

use archivefs::utils::filename_without_extension;

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
