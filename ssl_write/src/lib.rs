use std::{
    ffi::{c_char, c_int, c_void, CStr, CString},
    fs::File,
    io::Write,
    mem::transmute,
    process,
};

const RTLD_NEXT: *const c_void = -1isize as *const c_void;

fn dlsym_next(name: CString) -> *const c_void {
    unsafe { dlsym(RTLD_NEXT, name.as_ptr()) }
}

#[link(name = "dl")]
extern "C" {
    fn dlsym(handle: *const c_void, name: *const c_char) -> *const c_void;
}

#[no_mangle]
pub extern "C" fn SSL_write(ssl: *mut c_void, buf: *const c_void, num: c_int) -> c_int {
    let s = unsafe { CStr::from_ptr(buf as *const c_char) };
    let mut f = File::create("log.txt").unwrap();
    writeln!(f, "Process {}:\n{}", process::id(), s.to_string_lossy()).unwrap();

    let ssl_write_ptr = dlsym_next(CString::new("SSL_write").unwrap());
    let ssl_write_fn: fn(*mut c_void, *const c_void, c_int) -> c_int =
        unsafe { transmute(ssl_write_ptr) };
    ssl_write_fn(ssl, buf, num)
}
