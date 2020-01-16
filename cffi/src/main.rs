use std::ffi::{CStr, CString};
use std::os::raw::{c_double, c_schar};

// sudo apt get install libreadline-dev
#[link(name = "readline")]
// インポートするCの関数群を extern C ブロック内に書く
extern "C" {
    fn cos(x: c_double) -> c_double;
    fn readline(pronpt: *const c_schar) -> *mut c_schar;
}

fn main() {
    unsafe {
        println!("{}", cos(1.5));
    }
}

#[test]
fn c_cos() {
    unsafe {
        println!("{}", cos(1.5));
    }
}
