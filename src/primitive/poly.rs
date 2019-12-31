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

#[cfg(test)]
mod tests {
  use crate::primitive::poly::*;
  #[test]
  fn coord() {
    let c1 = CartesianCoord { x: 0.0, y: 0.0 };
    assert_eq!(CartesianCoord { x: 0.0, y: 0.0 }, c1.to_cartesian());
  }
}
