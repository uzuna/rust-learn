use std::ffi::{CStr, CString};
use std::os::raw::{c_double, c_int, c_schar, c_ulonglong, c_void};

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

#[link(name = "ownership", kind = "static")]
extern "C" {
    fn take_ownership(i: *const c_int, dtor: unsafe extern "C" fn(i: *mut c_int)) -> c_void;
}

// デストラクタ関数
// Cに渡した所有権をRustに戻してRustの所有権管理によって解放する
unsafe extern "C" fn drop_pointer(i: *mut c_int) {
    Box::from_raw(i);
}

fn main() {
    unsafe {
        println!("{}", cos(1.5));
        println!("fib(5) {}", fib(5));
    }

    // C primitive control
    {
        // xの参照からconstポインタは作れるが逆はできない
        let x = 1;
        let xptr: *const i32 = &x;
        // let xref: &i32 = xptr; // 不可

        // ポインタへの操作は基本的にunsafe
        unsafe {
            let x = *xptr;
        }

        // ミュータブル参照からはミュータブルなポインタが作れる。
        let mut y = 2;
        let yptr: *mut i32 = &mut y;
        unsafe {
            *yptr = 3;
        }

        // Boxからポインタが作れる
        let z = Box::new(4);
        let zptr: *const i32 = &*z;

        // 所有権のmoveを伴う場合
        // into_rawで変換し、from_rawで戻す
        // from_rawは二回呼べてrustの過程を破ってしまえるので要注意!
        let boxptr: *mut i32 = Box::into_raw(z);
        unsafe {
            // 他に参照がないかはユーザーが保証しなければならない
            let boxed = Box::from_raw(boxptr);
        }

        // Slice(文字列)からポインタが作れる
        let s: &[u8] = b"abc";
        let sptr: *const u8 = s.as_ptr();
        unsafe {
            // ポインタからスライスを作る場合はアンセーフ
            let s = std::slice::from_raw_parts(sptr, s.len());
        }
    }

    // Use C with Rust Destractor
    {
        for i in 0..1000 {
            let i = Box::new(i);
            unsafe { take_ownership(Box::into_raw(i), drop_pointer) };
        }
    }
}

#[test]
fn c_cos() {
    unsafe {
        println!("{}", cos(1.5));
    }
}
