fn main() {
    println!("cargo:rustc-link-search=native=/usr/local/opt/libarchive/lib");
    println!("cargo:rustc-link-lib=archive");
}
