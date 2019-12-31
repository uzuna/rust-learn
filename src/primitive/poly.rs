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
}
