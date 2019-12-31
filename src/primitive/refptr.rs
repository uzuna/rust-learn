// Reference Pointer
// 多くのケースでは一つのリソースに単一の所有者で済む
// しかしプログラムによっては複数の所有者を持たせるほうが設計的に良いケースもある
// それを実現する手段の紹介

#[cfg(test)]
mod tests {
  use crate::primitive::ownership::*;
  use std::rc::Rc;
  #[test]
  fn refptr_rc() {
    // Rc<T>ポインタとArc<T>ポインタ
    // ReferenceCountedの略で複数の所有者を持たせられるポインタ
    // 参照カウントをとり、0になったらリソースを消す
    let mut rc1 = Rc::new(Child(1));
    assert_eq!(1, Rc::strong_count(&rc1));
    {
      let rc2 = Rc::clone(&rc1);
      assert_eq!(2, Rc::strong_count(&rc1));
      assert_eq!(2, Rc::strong_count(&rc2));
    }
    assert_eq!(1, Rc::strong_count(&rc1));

    // 参照が1の時は可変の参照が得られる
    // そうでない場合はNone
    if let Some(child) = Rc::get_mut(&mut rc1) {
      child.0 += 1;
    } else {
      unreachable!();
    }
    // weekpointer?
    // 参照カウントは増えない
    let week = Rc::downgrade(&rc1);
    assert_eq!(1, Rc::strong_count(&rc1));

    // upgradeでアクセスできるようになる
    if let Some(rc3) = week.upgrade() {
      assert_eq!(2 as usize, rc1.0);
      assert_eq!(2, Rc::strong_count(&rc3));
    }
    // 参照を消すとupgrade出来なくなる
    std::mem::drop(rc1);
    if let Some(_) = week.upgrade() {
      unreachable!();
    }

    // week pointerを語るには参照ポインタの使い方について語る必要がある
    // 参照ポインタの場合は循環参照するとリソースが解放できなくなる
    // 相互に参照を持っており、どちらかを開放するともう一方が未定義参照を持つオブジェクトになってしまう
    // 弱い参照はupgradeしないと参照が得られないため未定義参照が出来ない。これはRc,Arcの概念でRustの物ではない
    // そういう場合、たとえば親子のリンクを表現するときにはP->Cを強C->Pを弱参照にするなどをする
    // 実現しているものによってはRcよりもアリーナアロケータなどのほうが良いこともある
  }

  use std::cell::RefCell;

  #[test]
  fn interior_mutability() {
    // コンパイル時の借用チェックを迂回する仕組み

    // この例だとStringは変更できないため以下のようにsフィールドだけを変更すrことができない
    struct A {
      c: char,
      s: String,
    }
    let a = A {
      c: 'a',
      s: "ales".to_string(),
    };
    let r = &a;
    // r.s.push('a'); // `r` is a `&` reference, so the data it refers to cannot be borrowed as mutable

    struct B {
      c: char,
      s: RefCell<String>,
    }

    let b = B {
      c: 'b',
      s: RefCell::new("alex".to_string()),
    };
    let rb = &b;
    rb.s.borrow_mut().push('a');
    {
      let rbs = b.s.borrow();
      assert_eq!(&*rbs, "alexa");
      // b.s.borrow_mut(); rbsが有効なのでここではmutが取れない
      assert!(b.s.try_borrow_mut().is_err());
    }
    assert!(b.s.try_borrow_mut().is_ok());
  }

  use std::collections::HashSet;
  #[test]
  fn tls_refcell() {
    // RefCellを使ったThred Local Storageの実装例
    // スレッドごとに個別の値を持つストレージを作る
    // thread_local!では不変の参照しか得られないが、内側のmutabilityで可変参照が使える
    thread_local!(
      static RABBITS: RefCell<HashSet<&'static str>> = {
        let rb = ["ロップイヤー","ダッチ"].iter().cloned().collect();
        RefCell::new(rb)
      }
    );
    RABBITS.with(|rb| {
      assert!(rb.borrow().contains("ロップイヤー"));
      rb.borrow_mut().insert("ネザーランド・ドワーフ");
    });
    // 別スレッドで生成して試すと
    // mainで入れた値は見つからないし
    // mainでも別スレッドで入れた値は見つからない
    std::thread::spawn(|| {
      // このスレッドでは初めて呼ばれたのでRABBITSは初期化されている
      RABBITS.with(|rb| {
        rb.borrow_mut().insert("ドワーフホト");
        assert!(!rb.borrow().contains("ネザーランド・ドワーフ"));
        assert!(rb.borrow().contains("ドワーフホト"));
      });
    })
    .join()
    .expect("Thread error");

    RABBITS.with(|rb| {
      assert!(rb.borrow().contains("ネザーランド・ドワーフ"));
      assert!(!rb.borrow().contains("ドワーフホト"));
    });
  }

  use std::error::Error;
  use std::sync::{Arc, RwLock};
  #[test]
  fn run_multi_thread_mutable() {
    arc_rwlock();
    arc_poisning();
  }

  fn arc_rwlock() -> Result<(), Box<dyn Error>> {
    // ArcとRwLockで複数スレッドで可変の値を共有する
    let cats: HashSet<_> = ["みけ", "ハチワレ", "サバ", "アメショ"]
      .iter()
      .cloned()
      .collect();
    let cats = Arc::new(RwLock::new(cats));

    fn stringify(x: impl ToString) -> String {
      x.to_string()
    }

    // csで参照をとっているブロック
    {
      let cs = cats.read().map_err(stringify)?;
      assert!(cs.contains("みけ"));
      assert!(cs.contains("ハチワレ"));
    }
    cats.write().map_err(stringify)?.insert("黒猫");

    // Arc::cloneでcatsへの参照ポインタを別のスレッドに渡している
    // ArcはRwLockへのDerefを実装しているためwrite()メソッドを直に呼べる
    // read,writeでスレッドをブロックする。ブロックしないtry_*()メソッドもある
    // read/writeはロックが取得できるとResult型を返す
    // 内部への参照はガード呼ばれるデータ構造を通してアクセスする
    // ガードはHashSetへのDerefを実装しているからそのままinsertを呼べる
    let cats1 = Arc::clone(&cats);
    std::thread::spawn(move || {
      cats1
        .write()
        .map(|mut cs| cs.insert("白猫"))
        .map_err(stringify)
    })
    .join()
    .expect("Thread error")?;
    assert!(cats.read().map_err(stringify)?.contains("みけ"));
    assert!(cats.read().map_err(stringify)?.contains("白猫"));
    Ok(())
  }

  fn arc_poisning() -> Result<(), Box<dyn Error>> {
    //
    let cats: HashSet<_> = ["みけ", "ハチワレ", "サバ", "アメショ"]
      .iter()
      .cloned()
      .collect();
    let cats = Arc::new(RwLock::new(cats));
    let cats1 = Arc::clone(&cats);
    std::thread::spawn(move || {
      let _guard = cats1.write();
      panic!();
    })
    .join()
    .expect_err("");
    fn stringify(x: impl ToString) -> String {
      x.to_string()
    }

    // writeをとったスレッドが落ちたばあい、arcの中身の整合性が取れない可能性がある
    // PoisonErrorとなっているためinto_inner()を使って中身を取り出すこともできる
    match cats.read() {
      Ok(_) => unreachable!(),
      Err(err) => {
        let data = err.into_inner();
        assert!(data.contains("みけ"));
        assert!(!data.contains("黒猫"));
      }
    }
    Ok(())
  }

  fn static_rwlock() {
    // crateが取れるようになったら自走する
  }

  #[test]
  fn closures() {
    // Closureは3つのトレイトの種類がある
    // fnは不変の環境を持つものでSyncを実装すれば複数スレッドで実行できる
    fn apply_fn<F>(f: &F, ch: char)
    where
      F: Fn(char) -> bool,
    {
      assert!(f(ch));
    }

    // fn_mutは可変の参照を持つ事を示し、すべての環境の値がSyncを実装しているときには
    // 複数スレッドで実行できる
    fn apply_fn_mut<F>(f: &mut F, ch: char)
    where
      F: FnMut(char) -> bool,
    {
      assert!(f(ch));
    }

    // 所有権をmoveさせるため1度しか実行できない
    fn apply_fn_once<F>(f: F, ch: char)
    where
      F: FnOnce(char) -> bool,
    {
      assert!(f(ch));
    }

    {
      let s1 = "read-only";
      let mut lookup = |ch| s1.find(ch).is_some();
      apply_fn(&lookup, 'r');
      apply_fn_mut(&mut lookup, 'o');
      apply_fn_once(lookup, 'y');
    }
    {
      // 可変の値を束縛する
      let mut s2 = "append".to_string();
      let mut modify = |ch| {
        s2.push(ch);
        true
      };
      // apply_fn(&modify, 'r'); Fnトレイトを実装していないので
      apply_fn_mut(&mut modify, 'e');
      apply_fn_once(modify, 'd');
      assert_eq!("appended", s2);
    }
    {
      // 可変の値を束縛する
      let mut s3 = "be converted".to_string();
      let mut consume = |ch| {
        let bytes = s3.into_bytes();
        bytes.contains(&(ch as u8))
      };
      // apply_fn(&modify, 'r'); Fnトレイトを実装していないので
      // apply_fn_mut(&mut consume, 'c'); // s3はinto_bytesで表示されるため
      apply_fn_once(consume, 'd');
      // assert_eq!("appended", s3); // error borrowd
    }

    // move keywordはクロージャの外からクロージャ環境へ持っていく
    // FnOnceはクロージャ環境から本体へ
    // std::thread::spawnは別スレッドで実行するため'static境界をもつ
    // いつ解放されるかわからない値を渡セルと問題があるから。
    // 参照を避けるためにmoveを使って別スレッドで'static扱いのライフタイムにできる
    let s1 = "read-only";
    let lookup = move || assert!(s1.find('d').is_some());
    let handle = std::thread::spawn(lookup);
    handle.join().expect("Failed to run thread.");
  }
}
