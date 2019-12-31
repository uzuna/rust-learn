// デカルト座標
#[derive(Debug, PartialEq)]
struct CartesianCoord {
  x: f64,
  y: f64,
}

// 極座標
#[derive(Debug, PartialEq)]
struct PolarCood {
  r: f64,
  theta: f64,
}

struct Matrix([[f64; 2]; 2]);

// traitの継承
// Coordinatesをすべて実装しているならLinucerTransformの実装を書いてもよいという指示
trait LinerTrasform: Coordinates {
  fn transform(self, matrix: &matrix) -> Self;
}

// traitの基本
// デカルト座標と相互変換できるものは座標トレイトを持つと定義
trait Coodinates {
  fn to_cartesian(self) -> CartesianCoord;
  fn from_catesian(cart: CartesianCoord) -> Self;
}

// traitの実装
impl Coodinates for CartesianCoord {
  fn to_cartesian(self) -> CartesianCoord {
    self
  }
  fn from_catesian(cart: CartesianCoord) -> Self {
    cart
  }
}

// Coodinatesを実装していない場合はエラーになる
impl LinerTrasform for CartesianCoord {
  fn transform(mut self, matrix: &Matrix) -> Self {
    let x = self.x;
    let y = self.y;
    let m = matrix.0;

    self.x = m[0][0] * x + m[0][1] * y;
    self.y = m[1][0] * x + m[1][1] * y;
    self
  }
}

impl Coodinates for PolarCood {
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
impl Coodinates for (f64, f64) {
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
fn print_point1<P: Coodinates>(point: P) {
  let p = point.to_cartesian(); // no methoad name
  println!("({}, {})", p.x, p.y);
}
// inline記法
fn print_point2<P>(point: P)
where
  P: Coodinates,
{
  let p = point.to_cartesian(); // no methoad name
  println!("({}, {})", p.x, p.y);
}
// impl Trait構文
fn print_point3(point: impl Coodinates) {
  let p = point.to_cartesian(); // no methoad name
  println!("({}, {})", p.x, p.y);
}

#[cfg(test)]
mod tests {
  use crate::primitive::poly::*;
  #[test]
  fn coord() {
    let p1 = (1.0, 0.0);
    let c1 = p1.to_cartesian();
    assert_eq!(CartesianCoord { x: 1.0, y: 0.0 }, c1);
    let p = PolarCood::from_catesian(c1);
    assert_eq!(PolarCood { r: 1.0, theta: 0.0 }, p);
    print_point1(p)
  }
}
