use better_cstr::c;

fn main() {
    c!(b"This byte string contains a null byte \0!");
}
