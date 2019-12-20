// Rustでは型とメモリ領域が密接に結びついている
// stack: 引数やローカル変数が置かれる
// heap: 動的に確保され、プログラム内で共有されるデータが置かれる
// Rustはデフォルトではstackに置き
// Box,VEc,Stringなどの可変長文字列はheapに置かれる

#[cfg(test)]
mod tests {

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
}
