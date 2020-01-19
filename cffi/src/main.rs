use std::ffi::{CStr, CString};
use std::os::raw::{c_double, c_int, c_schar, c_ulonglong};

// sudo apt get install libreadline-dev
#[link(name = "readline")]
// インポートするCの関数群を extern C ブロック内に書く
extern "C" {
    fn cos(x: c_double) -> c_double;
    fn readline(pronpt: *const c_schar) -> *mut c_schar;
}
#[link(name = "fib", kind = "static")]
extern "C" {
    fn fib(n: c_int) -> c_ulonglong;
}
fn main() {
    unsafe {
        println!("{}", cos(1.5));
        println!("fib(5) {}", fib(5));
    }
}

#[test]
fn c_cos() {
    unsafe {
        println!("{}", cos(1.5));
    }
}
