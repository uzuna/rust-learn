// lifetimeをVec<T>に似たベクタの実装で学ぶ
// 1. ベクタは連続したメモリ領域に格納し、この領域(elements)をヒープに確保する
// 2. pushメソッドで要素を追加できる。elementsのcapを超えた場合は現在の2倍を確保しなおす
// 3. get()による借用とpop()による取り出しをサポート
// 4. iter()をサポート

pub struct ToyVec<T> {
  // 簡易実装なのでヒープ領域の確保はBoxに任せる。(stdではallocaterを使ってる)
  // デメリットはデフォルト値を使うことにするためDefaultトレイト実装がある方だけ利用できる
  elements: Box<[T]>,
  len: usize,
}

impl<T: Default> ToyVec<T> {
  pub fn new() -> Self {
    Self::with_capacity(0)
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Self {
      elements: Self::allocate_in_heap(capacity),
      len: 0,
    }
  }

  pub fn len(&self) -> usize {
    self.len
  }
  pub fn capacity(&self) -> usize {
    self.elements.len()
  }

  pub fn push(&mut self, element: T) {
    if self.len == self.capacity() {
      self.grow();
    }
    self.elements[self.len] = element;
    self.len += 1;
  }
  pub fn get(&mut self, index: usize) -> Option<&T> {
    if index < self.len() {
      Some(&self.elements[index])
    } else {
      None
    }
  }

  // lifetime 指定子なしだと、deafult: &Tのライフタイムが不明になる
  // 'a, 'bとすると戻り値のライフタイムが合わない
  // ここではdefaultもselfと同じ'aライフタイムに制限することで解決する
  // where 'b: 'a と書いて 'bのほうが長いと示すことで解決する手もある
  pub fn get_or<'a>(&'a mut self, index: usize, default: &'a T) -> &'a T {
    // match self.get(index) {
    //   Some(v) => v,
    //   None => default,
    // }
    self.get(index).unwrap_or(default) // 上記のoneliner
  }

  pub fn pop(&mut self) -> Option<T> {
    if self.len == 0 {
      None
    } else {
      self.len -= 1;
      // これはコンパイルできない
      // &mut self 経由では所有権が取れない
      // cloneで実装もできるが物によってはコストが重いので避けたい
      // let elem = self.elements[self.len];
      // そのためDefault値と交換をする
      // String構造体だけが複製され、実際データのヒープは同じものがつかわれる
      let elem = std::mem::replace(&mut self.elements[self.len], Default::default());
      Some(elem)

      // std::mem::replaceはOption型でよく使われるためいかのメソッドが定義されている
      // 現在の値をムーブアウトして元の場所にNonを戻す
      // pub fn take(&mut self) -> Option<T>
      // 現在の値をムーブアウトして元の場所にSome(value)を残す
      // pub fn replace(&mut self, value: T) -> Option<T>;
    }
  }

  pub fn grow(&mut self) {
    if self.capacity() == 0 {
      self.elements = Self::allocate_in_heap(1);
    } else {
      // ここも同じく&mut selfから所有権を奪うために
      // std::mem::replaceで領域を交換し
      // into_vec().into_iter()でBoxからVecを通して所有権をとる
      // into_iter()は引数がselfでコレクションからmoveするため
      // old_elementsはアクセスできなくなる(今回はそれが良い)
      let new_elements = Self::allocate_in_heap(self.capacity() * 2);
      let old_elements = std::mem::replace(&mut self.elements, new_elements);
      for (i, elem) in old_elements.into_vec().into_iter().enumerate() {
        self.elements[i] = elem;
      }
    }

    // Vec<T>を含む標準ライブラリのコレクション型には
    // イテレータを得るために3つのメソッドがある
    // iter(&self)
    // iter_mut(&mut self)
    // into_iter(self)
    // 借用規則の参照がある場合に変更を許さないのは
    // 配列などでリアロケートされる可能性(要素が変わって再確保によって値のある位置が変わる)可能性があるため
  }

  fn allocate_in_heap(size: usize) -> Box<[T]> {
    std::iter::repeat_with(Default::default)
      .take(size)
      .collect::<Vec<_>>()
      .into_boxed_slice()
  }

  pub fn iter<'vec>(&'vec self) -> Iter<'vec, T> {
    Iter {
      elements: &self.elements,
      len: self.len,
      pos: 0,
    }
  }
}

impl<'vec, T: Default> IntoIterator for &'vec ToyVec<T> {
  type Item = &'vec T;
  type IntoIter = Iter<'vec, T>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}

pub struct Iter<'vec, T> {
  elements: &'vec Box<[T]>,
  len: usize,
  pos: usize,
}
// 関連型
impl<'vec, T> Iterator for Iter<'vec, T> {
  type Item = &'vec T;

  fn next(&mut self) -> Option<Self::Item> {
    if self.pos >= self.len {
      None
    } else {
      let res = Some(&self.elements[self.pos]);
      self.pos += 1;
      res
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::primitive::toyvec::ToyVec;
  #[test]
  fn run_basic() {
    let mut v = ToyVec::new();
    v.push("Java Finch".to_string());
    v.push("Budferifar".to_string());
    let e = v.get(1);
    assert_eq!(Some(&"Budferifar".to_string()), e);
  }
  #[test]
  fn run_long_lifetime() {
    // eをvより長い変数にすると
    let e: Option<&String>;
    {
      let mut v = ToyVec::new();
      v.push("Java Finch".to_string());
      v.push("Budferifar".to_string());

      // vよりeの生存期間が長くなってしまうためエラー
      // &Tが示すものself
      // e = v.get(1);
    }
    // assert_eq!(Some(&"Budferifar".to_string()), e);
  }
  #[test]
  fn run_iter_lifetime() {
    // eをvより長い変数にすると
    let e: Option<&String>;
    {
      let mut v = ToyVec::new();
      v.push("Java Finch".to_string());
      v.push("Budferifar".to_string());
      let mut iter = v.iter();

      // iterを使い終わるまではpush出来ない
      // v.push("Hill Mynah".to_string());
      assert_eq!(Some(&"Java Finch".to_string()), iter.next());
      v.push("Canary".to_string());
    }
  }
  #[test]
  fn run_into_iter() {
    // eをvより長い変数にすると
    let e: Option<&String>;
    {
      let mut v = ToyVec::new();
      v.push("Java Finch".to_string());
      v.push("Budferifar".to_string());
      for msg in &v {
        println!("{:?}", msg);
      }
    }
  }
}
