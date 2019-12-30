use std::ops::Drop;
#[derive(Debug)]
pub struct Parent(usize, Child, Child);

#[derive(Debug)]
pub struct Child(pub usize);

// デストラクタ
impl Drop for Parent {
  fn drop(&mut self) {
    println!("Dropping {:?}", self);
  }
}
impl Drop for Child {
  fn drop(&mut self) {
    println!("Dropping {:?}", self);
  }
}
#[derive(Copy, Clone, Debug)]
pub struct CloneableParent(usize, CloneableChild, CloneableChild);

#[derive(Copy, Clone, Debug)]
pub struct CloneableChild(usize);

#[cfg(test)]
mod tests {
  use crate::primitive::ownership::*;
  #[test]
  fn value_scope() {
    // p1がParentの所有者
    // ParentはChildの所有者
    // Vec<T>だったらVecがTを所有している
    let p1 = Parent(1, Child(11), Child(12));
    {
      let p2 = Parent(2, Child(21), Child(22));
      println!("(a) p1: {:?}, p2: {:?}", p1, p2);
    }
    println!("(a) p1: {:?}", p1);
  }

  #[test]
  fn resource_leak() {
    // 意図的にリソースリークできる
    // スタックにあるものは値が破棄されるが、Destructorが呼ばれない

    #[derive(Debug)]
    struct Resource(usize);
    impl Drop for Resource {
      fn drop(&mut self) {
        println!("unreachable! {:?}", self);
      }
    }

    let r1 = Resource(12);
    println!("living {:?}", r1);
    std::mem::forget(r1); // move

    // Box::into_raw
    // Box::leak
  }

  #[test]
  fn move_semantics() {
    let mut p1 = Parent(1, Child(11), Child(12));
    let p2 = p1; // 所有権が移る = スタック領域でデータはコピーされる
    assert_eq!(1, p2.0);
    // println!("p1: {:?}",p1); // borrow of moved value

    // 新しく所有権を持つためアクセスできるようになる
    p1 = Parent(2, Child(21), Child(22));
    assert_eq!(2, p1.0);
  }

  #[test]
  fn copy_cemantics() {
    // copy出来る条件は
    // 1. その構造体のすべてのフィールドがCopyトレイトを実装している
    // 2. その型とすべてのフィールドがDropトレイトを実装していない
    // 3. その型自身がCloneトレイトを実装している
    let mut p1 = CloneableParent(1, CloneableChild(11), CloneableChild(12));

    // copyの場合はp1の領域のデータは残ったまま
    // p2のスタックに値の複製が作られる
    let p2 = p1;
    assert_eq!(1, p2.0);
    p1.0 = 5;
    assert_eq!(5, p1.0);

    // CopyとCloneの違い
    // Copy: 暗黙的にバイト列でコピーするため早い
    // Clone: cloneメソッドで明示的にコピーされ、必要なロジックが組める
  }

  #[test]
  fn borrowing() {
    fn f1(p: Parent) {
      println!("p: {:?}", p);
    }
    fn f2(p: &mut Parent) {
      p.0 += 1;
    }

    let mut p1 = Parent(1, Child(11), Child(12));
    f1(p1);
    // assert_eq!(1, p1.0); // moveで渡してしまってすでに所有権を失っている。このままでは不便

    // 参照を渡すことで値そのものの所有権を保持したまま
    // 何かの処理を加えることができる
    p1 = Parent(1, Child(11), Child(12));
    f2(&mut p1);
    assert_eq!(2, p1.0);
    f2(&mut p1);
    assert_eq!(3, p1.0);
  }

  use std::collections::HashMap;

  #[test]
  fn borrowing_lifetime() {
    // 2種類の借用チェッカーがある
    // lexical scope
    // NLL=Non-Lexical-Lifetime

    // 2018 editionからNLLが有効になった
    // 制御フローグラフという中間表現であらわされ、match式などの表記でもRustの制御構造に適した借用の推測ができるようになった
    // 例えば下記のようにmatchの腕それぞれで借用変数が異なる例
    // map.get_mut() -> Some(value)
    // map.insert()とmap, valueの二つの異なる可変の参照を使っている
    // NLL以前はget_mutの借用がmatch式全体に及んでいるため
    // None腕の中でmap.insertができなかった

    fn process_or_default(key: char, map: &mut HashMap<char, String>) {
      match map.get_mut(&key) {
        Some(value) => value.push_str(", world!"),
        None => {
          map.insert(key, Default::default());
        }
      }
    }

    let mut map = HashMap::new();
    map.insert('h', "Hello".to_string());
    process_or_default('h', &mut map);
    process_or_default('v', &mut map);
    assert_eq!(Some(&String::from("Hello, world!")), map.get(&'h'));
    assert_eq!(Some(&String::from("")), map.get(&'v'));
  }
}
