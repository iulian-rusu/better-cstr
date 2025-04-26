# Better CStr Macro

[<img alt="github" src="https://img.shields.io/badge/github-iulian--rusu/better--cstr-874ac4?style=for-the-badge&logo=github" height="20">](https://github.com/iulian-rusu/better-cstr)
[<img alt="crates.io" src="https://img.shields.io/crates/v/better_cstr.svg?style=for-the-badge&color=de8912&logo=rust" height="20">](https://crates.io/crates/better_cstr)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-better_cstr-2bab82?style=for-the-badge&logo=docs.rs" height="20">](https://docs.rs/better_cstr)

Are you tired of writing `.as_ptr()` after all your C-string literals?

Simply use the `c!` macro:

```rs
use better_cstr::c;
use std::ffi::{c_char, CStr};

fn main() {
    let ptr: *const c_char = c!("Hello World");
    unsafe {
        println!("{:?}", CStr::from_ptr(ptr));
    }
}
```

Supports any kind of string literal that does not contain a null byte.
