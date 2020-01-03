# 模写

## bitonic sort

- `O(n log n)[worst]` でソートできるアルゴリズム
- データ数が2のべき乗でなければならない
  - 実際に使う時には緩和策としてダミーを入れてソート可能にするとか
  - 2べき数単位に分割してマージソートを行うなどがある 



## 気になるメモ

- memory copyされてるかどうかどうやって知る?
- benchmarkとかallocとかの測定はどこかにある?


# memo
## 所有権システム

### 利点

- GC不要。規模として小さくでき、OverHeadが少ない
- メモリ安全性がコンパイル時に保証される
- メモリ以外にもファイルやロックなども自動開放される

#### GC不要

GCはプログラム実行時に不定なメモリを確保開放するヒープ領域でメモリを管理する手法の一つ。手動で開放するのが難しいためGCによって自動開放をするというのがC以降の言語の多くで採用された。
一方でメモリを使っているかどうかを知るコスト、解放までのタイムラグが必要となる。
それを行うランタイムを含むため大きくなりがちでもあった。

所有権システムでは使用されるメモリ量が予測しやすく、即座に開放されるためGCというオーバーヘッドがなくなる

#### メモリ安全性がコンパイル時に保証される

Cはメモリ管理が開発者の手動のため性能の上限は高いがミスによるメモリ府安全が発生する。
具体的には以下の二つ
- 二重開放による未定義動作
- 不正なポインタを作らない

#### リソースの自動開放
GCはメモリ開放はしてくれるがリソース開放はしてくれない
リソース開放漏れがしばしばおこる
rustはメモリもリソースも同じ仕組みのため不要になった時点で自動開放となる

## 所有権システムの概要

#### 所有権

- 所有権はある値を所有する権利のこと。所有権を持つものをその値のownerと呼ぶ
- 変数が値の所有者になれるし、値自身も他の値の所有者になれる
- 値には所有者が一つだけある
- 所有者は値を指す不変、可変の参照を作ることで他社に値を貸し出せる
- 所有者は所有権と他人に譲渡できる
- 所有者がスコープを抜けるときに値のライフタイムが尽きる。そのタイミングで値は破棄され、リソースは解放される

#### move/copyセマンティクス

- 変数から別の変数に代入したとき、値の型によって意味(セマンティクス)が変わる
- ムーブでは所有権が変わる
- コピーでは複製される

#### 借用borrow

- 値を挿す参照を作ると、所有権の観点からは値を借用していることになる
- 借用には不変と可変の二種類がある

#### lifetime

- ライフタイムには値と参照の2種類がある
- 値は構築されたから破棄されるまで
- 参照は参照が使用される期間

#### 借用規則
所有権でメモリ安全を保障するための規則
- 不変可変問わず参照は値のスコープよりも短い
- 値が共有されている間は変更を許さない。以下の2つの状態のみしかない。
  - 任意個の不変の参照&Tを持つ
  - ただ一つの可変の参照mut &Tを持つ

## 値の所有者



## Trait

#### std::io::Write

`Vec<u8>` とかが実装していることで特別な型をつクラブにバイト列バッファとして利用可能にしている

#### std::convert::From

```rs
pub trait From<T>{
  fn from(T) -> Self;
}

// いろんな型に実装できる。一つのトレイトで多くの仕事ができる
impl From<u8> for u64 {...}
impl From<u16> for u64 {...}
impl From<u32> for u64 {...}
```

fromを実装するとintoメソッドが使えるようになるので多くの場合はintoメソッドとして使う
```rs
let string: String = "str".into();
```


#### std::iter::Iterator

```rs
pub trait Iterator {
  type Item;
  fn next(&mut self) -> Option<Self::Item>;

  fn size_hint(&self) -> (usize, Option<yser<>>)
  fn count(self) -> usize {}
}
```

#### std::ops::Eq

PartialEqを継承しているだけ。違いは `a=b`なら`b=a` が成り立つが `a=a` を保証しない。
Eqは保証する。具体的には f64のNanとか
メソッドは違っても別のセマンティクスを表現するのにつかう


#### std::marker::Sized
型のサイズがコンパイル時に決定できることを示す
`pub trait Sized{}`
コンパイラに特別扱いされる型のマーカーとしてもtraitが使われる


### 演算子のオーバーロード

トレイトで制御されている

```rs
1+1;
1.0+1.0;
"abc".to_string() + "def";
```

自分の型に実装するのは以下のようにやる

```rs
struct MyInt(i64);

impl Add<MyInt> for MyInt {
  type Output = Self;
  fn add(self, rhs MyInt) -> Self::Output {MyInt(self.0 + rhs.0)}
}
let one = MyInt(1);
let two = MyInt(2);
let mut i = one + two;
```

### トレイトのテクニック

#### StringとInto<String>

Stringを受け取る関数だと文字リテラルを渡すのに`.to_string()`を毎度呼ぶ必要がある

```rs
fn take_string(s: String){}
fn take_string(s: impl Into<String>){
  let _s = s.into();
}

take_string("some");
take_string("some".to_string().as_str()); // ゼロコスト抽象かなので変換されない
```

#### オプショナル引数

Fromトレイトが`impl<T> From<T> for Option<T>`のように実装されえているのでSomeが省略できる

```rs
fn range(min: impl Into<Option<usize>>, max: Into<Option<usize>>){}

range(1,None);
```

#### パスネーム

OSに依存するためRustの文字列と互換性があると限らない
`Path, OathBuf, &str, String, OSSttr, OSString` がパスネームっぽくふるまう
すべてAsRef<Path>を実装しているので以下のようなトレイト境界で統一的に扱える

```rs
fn hello_to_file(path: impl AsRef<Path>) -> io::Result<()> {
  let mut file = File::new(path.as_ref())?;
  write!(file,"Hello, File");
  Ok(())
}
```

#### &strとstr

strで実装するとレシーバが&strで使いやすくなる
BoxのDerefを通じて&strに変換できるのがよい

```rs
impl SomeTrait for str {
  fn take_ref(&self){}
}
let s = "hoge";
s.take_ref();
let box_s = Box::new(*s);
box_s.take_ref();
```

#### Newtypeによるtrait実装制約の回避

外部クレートの方に外部クレートのトレイトを実装できない話をした。
簡単に回避できる方法もあり一度タプルで包めばよい
wrapperなどの形で名前を適切につけること

```rs
struct ExLibStructWrapper(ExLibStruct);

impl ExLibTrait for ExLibStructWrapper{..}

let els =ExLibStructWrapper(ExLibStruct::new());
els.method();
```

#### 列挙型を使った型の混合

あるトレイトを実装した複数の方を混ぜて使うには、トレイトオブジェクトが必要だが、
登場する型をプログラムが把握していれば、トレイトオブジェクトを使わなくても列挙型で混合できる。

```rs
#[derive(Debug)]
enum Either<A, B> {
  A(A),
  B(B),
}

impl<A, B> fmt::Display for Either<A, B>
where 
  A: fmt::Display,
  B: fmt::Display,
{
  fn fmt(&self, f: &mut::Formatter) -> fmt::Result {
    match self {
      Either::A(a) => a.fmt(f),
      Either::B(b) => b.fmt(f),
    }
  }
}

let mut v : Vex<Either<bol, i32>> = vec![];
```

```


