// Rustでは型とメモリ領域が密接に結びついている
// stack: 引数やローカル変数が置かれる
// heap: 動的に確保され、プログラム内で共有されるデータが置かれる
// Rustはデフォルトではstackに置き
// Box,VEc,Stringなどの可変長文字列はheapに置かれる

#[cfg(test)]
mod tests {
  use std::collections::HashMap;
  #[test]
  fn r#box() {
    // stackに置かれる
    let t1 = (3, "birds".to_string());

    // b1ポインタがstackに作られ
    // heapの実態を所有する
    // t1は初期化される
    let mut b1 = Box::new(t1);
    (*b1).0 += 1;
    assert_eq!(*b1, (4, "birds".to_string()));
  }

  fn r#vec() {
    let v1 = vec![false, true, false];
    let v2 = vec![0.0, -1.0, 1.0, 0.5];

    assert_eq!(v1.len(), 3);
    assert_eq!(v2.len(), 4);

    let mut v6 = vec!['a', 'b', 'c'];
    v6.push('d');
    v6.push('e');
    assert_eq!(v6, ['a', 'b', 'c', 'd', 'e']);
    assert_eq!(v6.pop(), Some('e'));
    v6.insert(1, 'f'); // pos, value
    assert_eq!(v6.remove(2), 'f'); // pos return value

    let mut v7 = vec!['g', 'h'];
    v6.append(&mut v7);
    assert_eq!(v6, ['a', 'f', 'c', 'd', 'g', 'h']);
    assert_eq!(v7, []); //空になる

    let a8 = ['i', 'j'];
    v6.extend_from_slice(&a8);
    assert_eq!(v6, ['a', 'f', 'c', 'd', 'g', 'h', 'i', 'j']);
    assert_eq!(a8, ['i', 'j']);
  }

  #[test]
  fn collections() {
    let mut m1 = HashMap::new();

    m1.insert("a", 1);
    m1.insert("b", 3);
    assert_eq!(m1.len(), 2);

    assert_eq!(m1.get("b"), Some(&3));
    assert_eq!(m1.get("c"), None);

    // 無ければ追加するメソッドで参照を得る
    let d = m1.entry("d").or_insert(0);
    *d += 7;
    assert_eq!(m1.get("d"), Some(&7));

    // 配列を使った初期化
    let m2 = vec![("a", 1), ("b", 2)]
      .into_iter()
      .collect::<HashMap<_, _>>();

    assert_eq!(m2.get("b"), Some(&2));
  }

  #[test]
  fn type_string() {
    let mut s1 = "ラズベリー".to_string();
    let mut s2 = String::from("ブラックベリー");

    s1.push_str("タルト");
    assert_eq!(s1, "ラズベリータルト");

    s1.push('と');
    s1.push_str(&s2); //&strしか受け付けないので&をつけて型強制
    assert_eq!(s1, "ラズベリータルトとブラックベリー");

    let i = 42;
    assert_eq!(i.to_string(), "42");

    let f = 4.3 + 0.1;
    assert_eq!(f.to_string(), "4.3999999999999995");
    assert_eq!(format!("{:.2}", f), "4.40");
    let s1 = "44";
    assert_eq!(s1.parse::<i32>(), Ok(44));

    // 解釈できない場合はエラーとなる
    let s2 = "abc";
    let r2: Result<f64, _> = s2.parse();
    assert!(r2.is_err());
    println!("{:?}", r2);

    // from char
    let cs = ['t', 'r', 'u', 's', 't'];
    assert_eq!(cs.iter().collect::<String>(), "trust");
    assert_eq!(&cs[1..].iter().collect::<String>(), "rust");

    let bad_utf8: [u8; 7] = [
      b'a', // a
      0xf0, 0x90, 0x80, // でたらめなバイト列
      0xe3, 0x81, 0x82, // あ
    ];
    let s = String::from_utf8_lossy(&bad_utf8);
    assert_eq!(s, "a\u{fffd}あ"); // 不正なバイト列はfffd ReplacementCharacterにおきかわる

    // &strは生存期間の関係で返せない
    // Stringを返すのが一般的
    fn gen_string(name: &str) -> String {
      format!("Hello {}!", name)
    }
    assert_eq!(gen_string("world"), "Hello world!");

    // 他OSStringやCStrがある。
    // 状況に合わせて使う
  }

  #[test]
  fn std_ops_range() {
    // 範囲指定の構文
    let a = ['a', 'b', 'c', 'd', 'e'];

    assert_eq!(a[..], ['a', 'b', 'c', 'd', 'e']);
    assert_eq!(a[..3], ['a', 'b', 'c',]);
    assert_eq!(a[..=3], ['a', 'b', 'c', 'd',]);
    assert_eq!(a[1..], ['b', 'c', 'd', 'e']);
    assert_eq!(a[1..3], ['b', 'c',]);

    assert_eq!(.., std::ops::RangeFull);
    assert_eq!(..3, std::ops::RangeTo { end: 3 });
    assert_eq!(..=3, std::ops::RangeToInclusive { end: 3 });
    assert_eq!(1.., std::ops::RangeFrom { start: 1 });
    assert_eq!(1..3, std::ops::Range { start: 1, end: 3 });
    assert_eq!(1..=3, std::ops::RangeInclusive::new(1, 3));
  }

  #[test]
  fn std_option() {
    // Option型。値があるかどうかわからないことを示す型
    let a1 = ['a', 'b', 'c', 'd'];
    assert_eq!(a1.get(0), Some(&'a'));
    assert_eq!(a1.get(4), None);

    // 値の取り出しはmatchやif式が使える
    let mut o1 = Some(10);
    match o1 {
      Some(s) => assert_eq!(s, 10),
      None => unreachable!(),
    }

    o1 = Some(20);
    if let Some(s) = o1 {
      assert_eq!(s, 20);
    }

    // unwrap()持つかえるが、これはpanicを引き起こすので
    // 必要な場面以外ではunwrap_or_else()などを使う
    let mut o2 = Some(String::from("Hello"));
    assert_eq!(o2.unwrap(), "Hello");
    let o2 = None;
    // o2.unwrap();

    assert_eq!(
      o2.unwrap_or_else(|| String::from("o2 is none")),
      "o2 is none"
    );

    // Someの値を操作するときには map() や and_then()を使う

    // mapはSome()にたいしてクロージャを適用する
    let mut o3 = Some(25);
    let f = |n| n * 10;
    assert_eq!(o3.map(f), Some(250));

    o3 = None;
    assert_eq!(o3.map(f), None);

    // SomeかNoneかを返したい場合はand_then()
    o3 = Some(10);
    assert_eq!(
      o3.map(f)
        .and_then(|n| if n >= 200 { Some(n) } else { None }),
      None
    );

    // ? operator
    fn add_elems(s: &[i32]) -> Option<i32> {
      // ? opsはSomeなら値を取り出し、Noneなら関数からでる
      let s0 = s.get(0)?;
      let s3 = s.get(3)?;
      Some(s0 + s3)
    };
    assert_eq!(add_elems(&[3, 7, 31, 127]), Some(3 + 127));
    assert_eq!(add_elems(&[3, 7, 31]), None);
  }
}
