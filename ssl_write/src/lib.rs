use std::{
    ffi::{c_char, CStr},
    fs::File,
    io::Write,
    process,
};

#[no_mangle]
pub extern "C" fn rust_ssl_write(buffer: *const c_char) {
    let s = unsafe { CStr::from_ptr(buffer) };
    let mut f = File::create("log.txt").unwrap();
    write!(f, "Process {}:\n{}", process::id(), s.to_str().unwrap()).unwrap();
}
