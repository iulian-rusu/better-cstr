use better_cstr::c;
use std::ffi::{c_char, CStr};

#[test]
fn returns_pointer_to_cstr() {
    let ptr: *const c_char = c!("Hello World");

    unsafe {
        assert_eq!("Hello World", CStr::from_ptr(ptr).to_str().unwrap());
    }
}
