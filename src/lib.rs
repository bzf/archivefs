#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}

#[no_mangle]
pub extern "C" fn tell_me_things_rust() {
    println!("Hello from inside Rust!");
}
