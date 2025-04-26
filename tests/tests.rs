use better_cstr::c;
use std::ffi::{c_char, CStr};

#[test]
fn handles_valid_string_input() {
    let ptr: *const c_char = c!("Hello World");
    unsafe {
        assert_eq!("Hello World", CStr::from_ptr(ptr).to_str().unwrap());
    }
}

#[test]
fn handles_valid_raw_string_input() {
    let ptr: *const c_char = c!(r"Raw \0");

    unsafe {
        assert_eq!(r"Raw \0", CStr::from_ptr(ptr).to_str().unwrap());
    }
}

#[test]
fn handles_valid_byte_string_input() {
    let ptr: *const c_char = c!(b"Byte");

    unsafe {
        assert_eq!("Byte", CStr::from_ptr(ptr).to_str().unwrap());
    }
}

#[test]
fn fails_to_compile_on_invalid_input() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/invalid_*.rs");
}
