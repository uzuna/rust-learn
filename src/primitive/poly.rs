// デカルト座標
#[derive(Debug, PartialEq)]
pub struct CartesianCoord {
  x: f64,
  y: f64,
}

// 極座標
#[derive(Debug, PartialEq)]
pub struct PolarCood {
  r: f64,
  theta: f64,
}

// 自動導出derive(xxx)
// 可能な限りつけたほうが良いとされている
// 手続きマクロという機能で定義している

struct Matrix([[f64; 2]; 2]);

// traitの基本
// デカルト座標と相互変換できるものは座標トレイトを持つと定義
pub trait Coordinates {
  fn to_cartesian(self) -> CartesianCoord;
  fn from_catesian(cart: CartesianCoord) -> Self;
}

// traitの実装
impl Coordinates for CartesianCoord {
  fn to_cartesian(self) -> CartesianCoord {
    self
  }
  fn from_catesian(cart: CartesianCoord) -> Self {
    cart
  }
}
// traitの継承
// Coordinatesをすべて実装しているならLinucerTransformの実装を書いてもよいという指示
trait LinerTrasform: Coordinates {
  // デフォルト実装
  fn transform(self, matrix: &Matrix) -> Self
  where
    Self: Sized,
  {
    let mut cart = self.to_cartesian();
    let x = cart.x;
    let y = cart.y;
    let m = matrix.0;

    cart.x = m[0][0] * x + m[0][1] * y;
    cart.y = m[1][0] * x + m[1][1] * y;
    Self::from_catesian(cart)
  }

  // デフォルト実装
  fn rotate(self, theta: f64) -> Self
  where
    Self: Sized,
  {
    self.transform(&Matrix([
      [theta.cos(), -theta.sin()],
      [theta.sin(), theta.cos()],
    ]))
  }
}

// Coordinatesを実装していない場合はエラーになる
// Default実装だけで実装できる
impl LinerTrasform for CartesianCoord {}

// Coordinatesを実装していない場合はエラーになる
// rotateだけ上書きする
impl LinerTrasform for PolarCood {
  fn rotate(mut self, theta: f64) -> Self {
    self.theta += theta;
    self
  }
}

impl Coordinates for PolarCood {
  fn to_cartesian(self) -> CartesianCoord {
    CartesianCoord {
      x: self.r * self.theta.cos(),
      y: self.r * self.theta.sin(),
    }
  }
  fn from_catesian(cart: CartesianCoord) -> Self {
    PolarCood {
      r: (cart.x * cart.x + cart.y + cart.y).sqrt(),
      theta: (cart.y / cart.x).atan(),
    }
  }
}

// tupleにも実装できる
impl Coordinates for (f64, f64) {
  fn to_cartesian(self) -> CartesianCoord {
    CartesianCoord {
      x: self.0,
      y: self.1,
    }
  }
  fn from_catesian(cart: CartesianCoord) -> Self {
    (cart.x, cart.y)
  }
}

// Genericsで書いた場合はそのメソッドを持っていることを保証する境界が必要になる
// fn print_point<P>(point: P) {
//   let p = point.to_cartesian(); // no methoad name
//   println!("({}, {})", p.x, p.y);
// }
// Trait構文
fn print_point1<P: Coordinates>(point: P) {
  let p = point.to_cartesian(); // no methoad name
  println!("({}, {})", p.x, p.y);
}
// inline記法
fn print_point2<P>(point: P)
where
  P: Coordinates,
{
  let p = point.to_cartesian(); // no methoad name
  println!("({}, {})", p.x, p.y);
}
// impl Trait構文
fn print_point3(point: impl Coordinates) {
  let p = point.to_cartesian(); // no methoad name
  println!("({}, {})", p.x, p.y);
}

// Generics Trait
trait Init<T> {
  fn init(t: T) -> Self;
}

impl<T> Init<T> for Box<T> {
  fn init(t: T) -> Self {
    Box::new(t)
  }
}

trait As<T> {
  fn cast(self) -> T;
}

impl As<u64> for u8 {
  fn cast(self) -> u64 {
    self as u64
  }
}

// 同じAsをu8に実装しているがパラメータが異なるので問題ない
impl As<u32> for u8 {
  fn cast(self) -> u32 {
    self as u32
  }
}

#[cfg(test)]
mod tests {
  // 型を使う、またtrairのメソッドを呼ぶにはそのモジュールをインポートしてあることが必要
  // 実装にも制限がある。Traitを定義したクレート内或いは型を定義したクレート内でなければならない
  use crate::primitive::poly::*;

  #[test]
  fn coord() {
    let p1 = (1.0, 0.0);
    let c1 = p1.to_cartesian();
    assert_eq!(CartesianCoord { x: 1.0, y: 0.0 }, c1);
    let p = PolarCood::from_catesian(c1);
    assert_eq!(PolarCood { r: 1.0, theta: 0.0 }, p);
    print_point1(p);
    let p1 = (1.0, 0.0);
    let cr1 = p1.to_cartesian().rotate(std::f64::consts::PI * 1.0);
    assert_eq!(-1.0, cr1.x);
  }

  fn generics_trait() {
    // 推論可能な場合は省略できる
    let _data = Box::init("foo");
    // 型を指定する場合は型名::<型>と書く
    let _data = Box::<f32>::init(0.1);

    let _one_u32: u32 = 1.cast();
    let _one_u64: u64 = 1.cast();
    // let _one_i8: i8 = 1.cast(); // i8には実装していないので失敗する
  }

  use std::fmt::Display;
  #[test]
  fn dyn_dispatch() {
    // 静的ディスパッチと動的ディスパッチ
    // ゼロコスト抽象化のためげ練りクスは静的ディスパッチと呼ばれる方法で実現されている。
    // 静的に解決できるようにするための情報が増えるのでバイナリが大きくなるデメリットがある
    // dyn rait構文で動的ディスパッチもできる

    // コンパイル時に使用される型から専用のメソッドを作る。実行は早いが、使っている方の種類だけデータが増える
    // 静的ディスパッチのもう一つのデメリットはトレイトを実装した方を混ぜてVecに入れるなどができない
    // どの実態が来るかわからないからサイズが決まらないためVecを作ることすらできない
    // let mut v: Vec<Display> = vec![]; // doesn't have a size known at compile-time

    // Rustでは動的ディスパッチをトレイトオブジェクトで実現している
    // 他と違い違いトレイトオブジェクトはユーザーの定義したとは別に実装情報やデータサイズなどの内部情報を持つ
    // dyn Traitを使うことで表す
    // 関数ポインター経由で行うためジェネリクスに比べると多少コストがかかる
    let mut v: Vec<&dyn Display> = vec![];
    v.push(&true);
    v.push(&1i32);

    fn stringify(t: Box<dyn ToString>) -> String {
      t.to_string()
    }

    // トレードオフがあるから使い分ける
    // ほとんどの場合はジェネリクスで表現しきれないケースでのみ選ばれることが多い
    // トレイトオブジェクトを作るときもオブジェクト安全性と呼ばれる制約を満たさねばならない
  }

  #[test]
  fn impl_trait() {
    // 全称impl Trait: 引数の位置に書く
    // 存在impl Trait: 戻り値の位置に書く
    // 具体的な方には言及をせず、戻り値はIteratorを実装したなにかであると抽象化する
    fn to_n(n: i32) -> impl Iterator {
      0..n
    }

    // 複雑なイテレータを書くときにも役立つ
    // 下の場合戻り値の型はFilter<Range<i32>,fn(&i32) -> bool>だがIteratorと書けば済むし
    // 静的ディスパッチによって最適化されたコードが生成される
    fn to_n_even(n: i32) -> impl Iterator {
      (0..n).filter(|i| i % 2 == 0)
    }

    // クロージャなどの匿名型を返す場合は型がかけない
    // そこでトレイトオブジェクトか、impl Trailを使うことになる
    // fn gen_counter(init: i32) -> ??? {
    //   let mut n = init;
    //   move || {
    //     let res = n;
    //     n+=1;
    //     ret
    //   }
    // }
    // トレイトオブジェクトで書く場合
    fn gen_counter_trait_object(init: i32) -> Box<dyn FnMut() -> i32> {
      let mut n = init;
      Box::new(move || {
        let ret = n;
        n += 1;
        ret
      })
    }
    // 存在impl Traitの場合
    fn gen_counter_impl_trait(init: i32) -> impl FnMut() -> i32 {
      let mut n = init;
      move || {
        let ret = n;
        n += 1;
        ret
      }
    }
    let mut to1 = gen_counter_trait_object(1);
    assert_eq!(1, to1());
    assert_eq!(2, to1());
    let mut to2 = gen_counter_impl_trait(1);
    assert_eq!(1, to2());
    assert_eq!(2, to2());
  }
}
