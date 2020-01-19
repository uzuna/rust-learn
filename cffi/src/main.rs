use std::os::raw::{c_char, c_double, c_int, c_schar, c_ulonglong, c_void};

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
    fn make_memory() -> *mut c_int;
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
            let _x = *xptr;
        }

        // ミュータブル参照からはミュータブルなポインタが作れる。
        let mut y = 2;
        let yptr: *mut i32 = &mut y;
        unsafe {
            *yptr = 3;
        }

        // Boxからポインタが作れる
        let z = Box::new(4);
        let _zptr: *const i32 = &*z;

        // 所有権のmoveを伴う場合
        // into_rawで変換し、from_rawで戻す
        // from_rawは二回呼べてrustの過程を破ってしまえるので要注意!
        let boxptr: *mut i32 = Box::into_raw(z);
        unsafe {
            // 他に参照がないかはユーザーが保証しなければならない
            let _boxed = Box::from_raw(boxptr);
        }

        // Slice(文字列)からポインタが作れる
        let s: &[u8] = b"abc";
        let sptr: *const u8 = s.as_ptr();
        unsafe {
            // ポインタからスライスを作る場合はアンセーフ
            let _s = std::slice::from_raw_parts(sptr, s.len());
        }
    }

    // Use C with Rust Destractor
    {
        for i in 0..1 {
            let i = Box::new(i);
            unsafe { take_ownership(Box::into_raw(i), drop_pointer) };
        }
    }

    // Use rust with C malloc.flee
    {
        unsafe {
            let i = make_memory();
            println!("make_memory got {}", *i);
            libc::free(i as *mut _);
        }
    }
    //
    opaque();
    time_of_day();
}

// バリアントがない列挙型はユーザーが勝手に作れない
enum File {}

extern "C" {
    fn fopen(fname: *const c_char, mode: *const c_char) -> *mut File;
    fn fgetc(stream: *mut File) -> c_int;
    fn fclose(stream: *mut File) -> c_int;
}

fn opaque() {
    unsafe {
        let fname: *const c_char = b"Cargo.toml\0".as_ptr() as *const _;
        let mode: *const c_char = b"r\0".as_ptr() as *const _;
        let file = fopen(fname, mode);
        if file.is_null() {
            println!("open file failed");
            return;
        }
        loop {
            let c = fgetc(file);
            if c == -1 {
                println!("failed to get");
                break;
            } else {
                let c = c as u8 as char;
                print!("{}", c);
            }
        }
        if fclose(file) == -1 {
            println!("close file failed");
        }
    }
}

use libc::{suseconds_t, time_t};
use std::mem;
use std::ptr;

#[repr(C)]
#[derive(Debug)]
struct Timeval {
    tv_sec: time_t,       /* second */
    tv_usec: suseconds_t, /* */
}

#[repr(C)]
#[derive(Debug)]
struct Timezone {
    tz_minuteswest: c_int, /* second */
    tz_dsttime: c_int,     /* */
}

extern "C" {
    fn gettimeofday(tv: *mut Timeval, tz: *mut Timezone) -> c_int;
}

fn time_of_day() {
    unsafe {
        let mut tv: Timeval = mem::uninitialized();
        let tz: *mut Timezone = ptr::null_mut();
        let ret = gettimeofday(&mut tv as *mut _, tz);
        if ret == -1 {
            println!("failure");
            return;
        }
        println!("{:?}", tv);

        // これの注意点は*mutや*constはNullが返ってくる可能性があること
    }
}

#[test]
fn c_cos() {
    unsafe {
        println!("{}", cos(1.5));
    }
}
