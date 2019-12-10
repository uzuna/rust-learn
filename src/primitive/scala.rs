// unit is ()
// this example return "unit"
fn hello_unit() {
  println!("call hello_unit!");
}

fn str_append(s1: &str) -> String {
  let s2 = "world!";
  let s3 = s1.to_string() + s2;
  assert_eq!(s3, "Hello, world!");
  s3
}

fn add_value(a: i32, b: i32) -> i32 {
  a + b
}

// 短いライフタイムのrefは上に渡せない
// fn add_ref<'a>(a: &'a i32, b: &'a i32) -> &'a i32 {
//   &(*a + *b);
// }

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn return_unit() {
    let ret = hello_unit();
    assert_eq!(ret, ()); // true
    assert_eq!(std::mem::size_of::<()>(), 0); // メモリの大きさは0バイト
    ()
  }

  #[test]
  fn return_bool() {
    let b1 = true;
    let b2 = !b1;
    assert_eq!(b2, false);

    let n1 = 8;
    let n2 = 12;
    assert_eq!(n1 >= 10, false);
    assert_eq!(n2 >= 10, true);
    assert_eq!(b2 && b1, false);
    assert_eq!(b2 || b1, true);
    assert_eq!(std::mem::size_of::<bool>(), 1);
  }

  #[test]
  fn return_int() {
    let n1 = add_value(10_000, 6);
    assert_eq!(n1, 10_006);

    let _n2 = 0u8; // u8 by suffix
    let n3 = -100_isize; // isize
    let n4 = 10; // 型推論 isize
    assert_eq!(n3 + n4, -90);

    let _h1 = 0xff;
    let _o1 = 0o744;
    let _b1 = 0b0101_0110_1110_1001;

    assert_eq!(b'A', 65u8); // "A"の文字コード
  }

  #[test]
  fn xor_operator() {
    let b1 = 0b0110_0111;
    let b2 = 0b0011_1110;
    assert_eq!(b1 ^ b2, 0b0101_1001); // XOR
  }
  #[test]
  fn math_fn() {
    let n1 = 200u8;
    let n2 = 3u8;
    // let n3 = n1+n2;// panic! run on debug mode
    // 桁あふれはdebug modeでパニックになる
    // 桁あふれに対応した関数がある

    assert_eq!(n1.checked_mul(n2), None);
    assert_eq!(n1.saturating_mul(n2), std::u8::MAX);
    assert_eq!(n1.wrapping_mul(n2), 88);
    assert_eq!(n1.overflowing_mul(n2), (88, true));
  }

  #[test]
  fn return_str() {
    let s1 = "Hello, ";
    let s2 = str_append(s1);
    assert_eq!(s2, "Hello, world!");
  }
}
