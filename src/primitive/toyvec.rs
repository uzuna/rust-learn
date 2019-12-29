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
  pub fn grow(&mut self) {
    if self.capacity() == 0 {
      self.elements = Self::allocate_in_heap(1);
    } else {
      let new_elements = Self::allocate_in_heap(self.capacity() * 2);
      let old_elements = std::mem::replace(&mut self.elements, new_elements);
      for (i, elem) in old_elements.into_vec().into_iter().enumerate() {
        self.elements[i] = elem;
      }
    }
  }

  fn allocate_in_heap(size: usize) -> Box<[T]> {
    std::iter::repeat_with(Default::default)
      .take(size)
      .collect::<Vec<_>>()
      .into_boxed_slice()
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
}
