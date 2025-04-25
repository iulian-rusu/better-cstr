use better_cstr::c;
use std::ffi::{c_char, CStr};

#[test]
fn given_valid_string_returns_pointer_to_cstr() {
    let ptr: *const c_char = c!("Hello World");

    unsafe {
        assert_eq!("Hello World", CStr::from_ptr(ptr).to_str().unwrap());
    }
}

#[test]
fn given_valid_raw_string_returns_pointer_to_cstr() {
    let ptr: *const c_char = c!(r"Raw \0");

    unsafe {
        assert_eq!(r"Raw \0", CStr::from_ptr(ptr).to_str().unwrap());
    }
}

#[test]
fn given_valid_byte_string_returns_pointer_to_cstr() {
    let ptr: *const c_char = c!(b"Byte");

    unsafe {
        assert_eq!("Byte", CStr::from_ptr(ptr).to_str().unwrap());
    }
}

#[test]
fn invalid_literal_fails_to_compile() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/compile_fail/invalid_lit*.rs");
}
