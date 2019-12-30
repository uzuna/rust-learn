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
  }
}
