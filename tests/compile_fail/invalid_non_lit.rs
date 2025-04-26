use better_cstr::c;

fn main() {
    c!(std::ops::Add::add(34, 35));
}
