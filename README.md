# Better CStr Macro

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
