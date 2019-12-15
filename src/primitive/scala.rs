use std::str::FromStr;

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

    let f1 = 10.0f64; //f64
    let f2 = 1_234.567f64;
    let _f3 = 578.6E+77;

    assert_eq!(f1.mul_add(f2, 1.11), 12_346.78f64);
    assert_eq!(f1.max(f2), f2);
    println!("pow 2: {}", f1.powi(2));
    println!("pow 1.5: {}", f1.powf(1.5));
    println!("sqrt: {}", f1.sqrt());
    println!("log: {}", f1.log(2.0));

    let x = std::f64::consts::FRAC_PI_2;
    println!("sin: {}", x.sin());
    println!("cos: {}", x.cos());
    println!("asin: {}", x.asin());
    println!("acos: {}", x.acos());
    println!("atan: {}", x.atan());

    let x1 = 3.0_f64;
    let y1 = -3.0_f64;
    println!("atan2: {}", x1.atan2(y1));

    let f4 = 1.5f64;
    assert_eq!(f4.ceil(), 2.0);
    assert_eq!(f4.round(), 2.0);
    assert_eq!(f4.floor(), 1.0);

    let fna: f64 = 0.0 / 0.0;
    assert!(fna.is_nan());
    assert!(f4.is_finite());
    assert!(std::f64::INFINITY.is_infinite());
    assert!(y1.is_sign_negative());
    assert!(x1.is_sign_positive());

    // with string
    let s = "1234.567";
    let x5 = f64::from_str(s).unwrap();
    assert_eq!(x5, 1_234.567f64);
    assert_eq!(x5.to_string(), s);

    // consts
    println!("e = {}", std::f64::consts::E);
  }

  #[test]
  fn test_char() {
    let c1 = 'A';
    let c2 = 'a';
    assert!(c1 < c2);
    assert!(c1.is_uppercase());

    let c3 = '9';
    assert!(c3.is_digit(10)); // 10進数
                              // assert!(c3.is_digit(8)); // error
    let _c4 = '\t';
    let _c5 = '\n';
    let _c6 = '\'';
    let _c9 = '漢';
    let _c10 = '\u{5b57}';
    let _c11 = '\u{1f600}';
    assert_eq!(std::mem::size_of::<char>(), 4);
  }

  #[test]
  fn reference() {
    fn f1(mut n: i32) -> i32 {
      n += 1;
      n
    }
    fn f2(n: &mut i32) -> i32 {
      *n += 1;
      *n
    }
    let n = 1;
    assert_eq!(f1(n), 2);
    assert_eq!(n, 1);
    let mut n = 1i32;
    assert_eq!(f2(&mut n), n);
  }

  #[test]
  fn raw_pointer() {
    // メモリ安全ではないポインタで
    // *const T か *mut T となる
    let c1 = 'A';
    let c1_ptr: *const char = &c1; // 明示的に変換
    assert_eq!(unsafe { *c1_ptr }, 'A'); // 生ポインターの参照外しはunsafe

    let mut n1 = 0;
    let n1_ptr: *mut i32 = &mut n1;
    assert_eq!(unsafe { *n1_ptr }, 0);
    unsafe {
      *n1_ptr = 1_000;
      assert_eq!(*n1_ptr, 1_000);
    }
  }

  #[test]
  fn fn_pointer() {
    fn double(n: i32) -> i32 {
      n + n
    }
    fn abs(n: i32) -> i32 {
      if n >= 0 {
        n
      } else {
        -n
      }
    }

    let mut f: fn(i32) -> i32 = double;
    // let mut f = double; // 型注釈なしでは関数定義と型推論されてエラーになる
    assert_eq!(f(-42), -84);

    f = abs;
    assert_eq!(f(-42), 42);

    // 関数ポインタのサイズはusizeと同じ
    assert_eq!(std::mem::size_of_val(&f), std::mem::size_of::<usize>());

    // closure
    // コンパイル時に必要な変数を束縛した独自の環境を生成してそこで実行される
    // ゆえにクロージャは関数ごとに独自の型が作られて扱われるため
    // クロージャを受け取る関数の場合にはGenericsを使う必要がある
    let x = 4;
    let adder = |n| n + x;
    assert_eq!(adder(2), 6);

    // 束縛した情報を書き換えられるクロージャ
    let mut state = false;
    let mut flipflop = || {
      state = !state;
      state
    };

    assert_eq!(flipflop(), true);
    assert_eq!(flipflop(), false);
    assert_eq!(flipflop(), true);
    assert_eq!(state, true);

    // クロージャはFn, FnMut, FnOnceトレイトをもつ
    // unsafeのつかない関数ポインタもそれらを自動実装する
    // なのでmapの中にクロージャーをわたす、或いは関数ポインタを渡すの両方ができる
    let v = vec!["I", "love", "Rust!"]
      .into_iter()
      .map(|s| s.len()) // 文字列を受け取るクロージャ
      .collect::<Vec<_>>();
    assert_eq!(v, vec![1, 4, 5]);
    let v = vec!["I", "love", "Rust!"]
      .into_iter()
      .map(str::len) // 自動実装のため関数ポインターを渡すことができる
      .collect::<Vec<_>>();
    assert_eq!(v, vec![1, 4, 5]);
  }

  #[test]
  fn tuple() {
    // 要素番号から取り出すことができる
    let t1 = (88, true);
    assert_eq!(t1.0, 88);
    assert_eq!(t1.1, true);

    // 変数を使うことはできない
    // let i = 0;
    // let t1a = 1t.1; // compile error

    let mut t1 = (88, true);
    t1.0 += 100;
    assert_eq!(t1, (188, true));

    // pattern matchを使って分解
    let (n1, n2) = t1;
    assert_eq!(n1, 188);
    assert_eq!(n2, true);

    // 要素を指す可変の参照を得る
    // _ で無視もできる
    let mut t1 = ((0, 5), (10, -1));
    let ((ref mut x1_ptr, ref mut y1_ptr), _) = t1;

    *x1_ptr += 3;
    *y1_ptr *= -1;
    assert_eq!(t1, ((3, -5), (10, -1)));
  }

  #[test]
  fn return_str() {
    let s1 = "Hello, ";
    let s2 = str_append(s1);
    assert_eq!(s2, "Hello, world!");
  }
}
