use std::os::raw::c_double;

// インポートするCの関数群を extern C ブロック内に書く
extern "C" {
    fn cos(x: c_double) -> c_double;
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
