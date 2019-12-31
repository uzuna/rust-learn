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
}
