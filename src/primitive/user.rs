// Rustでは型とメモリ領域が密接に結びついている
// stack: 引数やローカル変数が置かれる
// heap: 動的に確保され、プログラム内で共有されるデータが置かれる
// Rustはデフォルトではstackに置き
// Box,VEc,Stringなどの可変長文字列はheapに置かれる

#[cfg(test)]
mod tests {
  use std::collections::HashMap;
  #[test]
  fn r#box() {
    // stackに置かれる
    let t1 = (3, "birds".to_string());

    // b1ポインタがstackに作られ
    // heapの実態を所有する
    // t1は初期化される
    let mut b1 = Box::new(t1);
    (*b1).0 += 1;
    assert_eq!(*b1, (4, "birds".to_string()));
  }

  fn r#vec() {
    let v1 = vec![false, true, false];
    let v2 = vec![0.0, -1.0, 1.0, 0.5];

    assert_eq!(v1.len(), 3);
    assert_eq!(v2.len(), 4);

    let mut v6 = vec!['a', 'b', 'c'];
    v6.push('d');
    v6.push('e');
    assert_eq!(v6, ['a', 'b', 'c', 'd', 'e']);
    assert_eq!(v6.pop(), Some('e'));
    v6.insert(1, 'f'); // pos, value
    assert_eq!(v6.remove(2), 'f'); // pos return value

    let mut v7 = vec!['g', 'h'];
    v6.append(&mut v7);
    assert_eq!(v6, ['a', 'f', 'c', 'd', 'g', 'h']);
    assert_eq!(v7, []); //空になる

    let a8 = ['i', 'j'];
    v6.extend_from_slice(&a8);
    assert_eq!(v6, ['a', 'f', 'c', 'd', 'g', 'h', 'i', 'j']);
    assert_eq!(a8, ['i', 'j']);
  }

  #[test]
  fn collections() {
    let mut m1 = HashMap::new();

    m1.insert("a", 1);
    m1.insert("b", 3);
    assert_eq!(m1.len(), 2);

    assert_eq!(m1.get("b"), Some(&3));
    assert_eq!(m1.get("c"), None);

    // 無ければ追加するメソッドで参照を得る
    let d = m1.entry("d").or_insert(0);
    *d += 7;
    assert_eq!(m1.get("d"), Some(&7));

    // 配列を使った初期化
    let m2 = vec![("a", 1), ("b", 2)]
      .into_iter()
      .collect::<HashMap<_, _>>();

    assert_eq!(m2.get("b"), Some(&2));
  }

  #[test]
  fn type_string() {
    let mut s1 = "ラズベリー".to_string();
    let mut s2 = String::from("ブラックベリー");

    s1.push_str("タルト");
    assert_eq!(s1, "ラズベリータルト");

    s1.push('と');
    s1.push_str(&s2); //&strしか受け付けないので&をつけて型強制
    assert_eq!(s1, "ラズベリータルトとブラックベリー");

    let i = 42;
    assert_eq!(i.to_string(), "42");

    let f = 4.3 + 0.1;
    assert_eq!(f.to_string(), "4.3999999999999995");
    assert_eq!(format!("{:.2}", f), "4.40");
    let s1 = "44";
    assert_eq!(s1.parse::<i32>(), Ok(44));

    // 解釈できない場合はエラーとなる
    let s2 = "abc";
    let r2: Result<f64, _> = s2.parse();
    assert!(r2.is_err());
    println!("{:?}", r2);

    // from char
    let cs = ['t', 'r', 'u', 's', 't'];
    assert_eq!(cs.iter().collect::<String>(), "trust");
    assert_eq!(&cs[1..].iter().collect::<String>(), "rust");

    let bad_utf8: [u8; 7] = [
      b'a', // a
      0xf0, 0x90, 0x80, // でたらめなバイト列
      0xe3, 0x81, 0x82, // あ
    ];
    let s = String::from_utf8_lossy(&bad_utf8);
    assert_eq!(s, "a\u{fffd}あ"); // 不正なバイト列はfffd ReplacementCharacterにおきかわる

    // &strは生存期間の関係で返せない
    // Stringを返すのが一般的
    fn gen_string(name: &str) -> String {
      format!("Hello {}!", name)
    }
    assert_eq!(gen_string("world"), "Hello world!");

    // 他OSStringやCStrがある。
    // 状況に合わせて使う
  }

  #[test]
  fn std_ops_range() {
    // 範囲指定の構文
    let a = ['a', 'b', 'c', 'd', 'e'];

    assert_eq!(a[..], ['a', 'b', 'c', 'd', 'e']);
    assert_eq!(a[..3], ['a', 'b', 'c',]);
    assert_eq!(a[..=3], ['a', 'b', 'c', 'd',]);
    assert_eq!(a[1..], ['b', 'c', 'd', 'e']);
    assert_eq!(a[1..3], ['b', 'c',]);

    assert_eq!(.., std::ops::RangeFull);
    assert_eq!(..3, std::ops::RangeTo { end: 3 });
    assert_eq!(..=3, std::ops::RangeToInclusive { end: 3 });
    assert_eq!(1.., std::ops::RangeFrom { start: 1 });
    assert_eq!(1..3, std::ops::Range { start: 1, end: 3 });
    assert_eq!(1..=3, std::ops::RangeInclusive::new(1, 3));
  }

  #[test]
  fn std_option() {
    // Option型。値があるかどうかわからないことを示す型
    let a1 = ['a', 'b', 'c', 'd'];
    assert_eq!(a1.get(0), Some(&'a'));
    assert_eq!(a1.get(4), None);

    // 値の取り出しはmatchやif式が使える
    let mut o1 = Some(10);
    match o1 {
      Some(s) => assert_eq!(s, 10),
      None => unreachable!(),
    }

    o1 = Some(20);
    if let Some(s) = o1 {
      assert_eq!(s, 20);
    }

    // unwrap()持つかえるが、これはpanicを引き起こすので
    // 必要な場面以外ではunwrap_or_else()などを使う
    let mut o2 = Some(String::from("Hello"));
    assert_eq!(o2.unwrap(), "Hello");
    let o2 = None;
    // o2.unwrap();

    assert_eq!(
      o2.unwrap_or_else(|| String::from("o2 is none")),
      "o2 is none"
    );

    // Someの値を操作するときには map() や and_then()を使う

    // mapはSome()にたいしてクロージャを適用する
    let mut o3 = Some(25);
    let f = |n| n * 10;
    assert_eq!(o3.map(f), Some(250));

    o3 = None;
    assert_eq!(o3.map(f), None);

    // SomeかNoneかを返したい場合はand_then()
    o3 = Some(10);
    assert_eq!(
      o3.map(f)
        .and_then(|n| if n >= 200 { Some(n) } else { None }),
      None
    );

    // ? operator
    fn add_elems(s: &[i32]) -> Option<i32> {
      // ? opsはSomeなら値を取り出し、Noneなら関数からでる
      let s0 = s.get(0)?;
      let s3 = s.get(3)?;
      Some(s0 + s3)
    };
    assert_eq!(add_elems(&[3, 7, 31, 127]), Some(3 + 127));
    assert_eq!(add_elems(&[3, 7, 31]), None);
  }

  #[test]
  fn std_result() {
    // 結果がエラーになる可能性を暗示する型
    // 列挙型で定義されていてOkとなるTかErrのいずれかを返す
    // Errの詳細を伝えたいときにはOptionではなくResultを使う

    assert_eq!("10".parse::<i32>(), Ok(10));
    let res0 = "a".parse::<i32>();
    assert!(res0.is_err());
    println!("{:?}", res0);

    // ? operatorが使える
    // try!マクロは以前使われていたが、1.13からは?が推奨
    fn add0(s0: &str, s1: &str) -> Result<i32, std::num::ParseIntError> {
      let s0 = s0.parse::<i32>()?;
      let s1 = s1.parse::<i32>()?;
      Ok(s0 + s1)
    }

    assert_eq!(add0("3", "27"), Ok(30));
    assert!(add0("3", "abc").is_err());

    // Optionと同じくmap,and_then,or_elseが使え
    // エラーの書き換えを行うmap_errも使える
    fn add1(s0: &str, s1: &str) -> Result<i32, String> {
      let s0 = s0.parse::<i32>().map_err(|_e| "s0が整数ではありません")?;
      let s1 = s1.parse::<i32>().map_err(|_e| "s1が整数ではありません")?;
      Ok(s0 + s1)
    }

    assert_eq!(add1("3", "27"), Ok(30));
    assert_eq!(add1("3", "abc"), Err("s1が整数ではありません".to_string()));
  }

  #[test]
  fn type_alias() {
    type UserName = String;
    type Id = i64;
    type Timestamp = i64;
    type User = (Id, UserName, Timestamp);

    fn new_user(name: UserName, id: Id, created: Timestamp) -> User {
      (id, name, created)
    }

    let id = 400;
    let now = 4567890123;
    let user = new_user(String::from("mika"), id, now);
    assert_eq!(user.1, "mika".to_string());

    // 新しい方ではないのでi64の項目が入れ替わってもコンパイルは通る
    // let bad_user = new_user(String::from("mika"), now, id);

    // 型aliasは型のネストが深くなった時に使うと便利
    type SharedMap<K, V> = std::rc::Rc<std::cell::RefCell<std::collections::HashMap<K, V>>>;

    // 型パラメータの具象化にも
    // type Result<T> = result::Result<T,Error>;
  }

  #[test]
  fn type_struct() {
    // 名前付きフィールド構造体
    // Default deriveによって型に合わせた初期値を自動導出できる
    // そうしなければフィールドを省略した初期化はできない
    // #[derive(Default)]
    struct Polygon {
      vertexts: Vec<(i32, i32)>,
      stroke_width: u8,
      fill: (u8, u8, u8),
    }

    // 独自に初期値を設定したい場合はdefault implを実装する
    impl Default for Polygon {
      fn default() -> Self {
        Self {
          stroke_width: 1,
          vertexts: Default::default(),
          fill: Default::default(),
        }
      }
    }

    // 値の初期化
    let triangle = Polygon {
      vertexts: vec![(0, 0), (3, 0), (2, 2)],
      fill: (255, 255, 255),
      stroke_width: 1,
    };

    // フィールド名と同じローカル変数の場合はフィールド名を省略できる
    fn new_polygon(vertexts: Vec<(i32, i32)>) -> Polygon {
      let stroke_width = 1;
      let fill = (0, 0, 0);
      Polygon {
        vertexts,
        fill,
        stroke_width,
      }
    }
    let quadrangle = new_polygon(vec![(5, 2), (4, 7), (10, 6), (8, 1)]);

    // .<field_name>でアクセスできる
    assert_eq!(triangle.vertexts[0], (0, 0));
    assert_eq!(triangle.vertexts.len(), 3);
    assert_eq!(triangle.fill, (255, 255, 255));

    // パターンマッチによるアクセス
    let Polygon {
      vertexts: quad_vx, ..
    } = quadrangle;
    assert_eq!(4, quad_vx.len());
    // パターンマッチによるアクセス same field name
    let Polygon { fill, .. } = quadrangle;
    assert_eq!(fill, (0, 0, 0));

    // 書き換えはmutである場合にできる
    let mut polygon = new_polygon(vec![(-1, -5), (-4, 0)]);
    assert_eq!(polygon.vertexts.len(), 2);
    polygon.vertexts.push((2, 8));
    assert_eq!(polygon.vertexts.len(), 3);

    assert_eq!(triangle.stroke_width, 1);
    let triangle1 = Polygon {
      stroke_width: 5,
      ..triangle
    };
    // assert_eq!(triangle1.fill, triangle1.fill); //moved!!
    assert_eq!(triangle1.fill, (255, 255, 255));
    assert_eq!(triangle1.stroke_width, 5);

    let polygon1: Polygon = Default::default();
    let polygon2 = Polygon {
      vertexts: vec![(0, 0), (3, 0), (2, 2)],
      ..Default::default()
    };
    assert_eq!(polygon1.fill, (0, 0, 0));
    assert_eq!(polygon2.stroke_width, 1);

    // タプル構造体
    struct Triangle(Vertex, Vertex, Vertex);
    struct Vertex(i32, i32);

    let vx0 = Vertex(0, 0);
    let vx1 = Vertex(3, 0);
    let triangle = Triangle(vx0, vx1, Vertex(2, 2));

    assert_eq!((triangle.1).0, 3);

    // 型エイリアスでは元の方が同じならエラーにならなかった
    // 代わりに1つのタプル構造体を使うとエラーが出るようになる
    struct UserName(String);
    struct Id(u64);
    struct Timestamp(u64);
    type User = (Id, UserName, Timestamp);

    fn new_user(name: UserName, id: Id, created: Timestamp) -> User {
      (id, name, created)
    }
    let id = Id(400);
    let now = Timestamp(45678901234);

    // let bad_user = new_user(UserName(String::from("kazuki")), now, id); // mismatched type
    let _user = new_user(UserName(String::from("kazuki")), id, now);

    // ユニット構造体
    #[derive(Debug, PartialEq)]
    struct UniqueValue;

    // 取りうる値は一つしかない
    // フィールドとして持つ値がないもののトレイトを実装する時とかにつかう
    let uv1 = UniqueValue;
    let uv2 = UniqueValue;
    assert_eq!(uv1, uv2);
  }

  #[test]
  fn type_enum() {
    #[derive(Debug, PartialEq)]
    enum Weekday {
      Monday,
      Tuesday,
      Wednesday,
      Tursday,
      Friday,
    }
    fn say_something(weekday: Weekday) {
      if weekday == Weekday::Friday {
        println!("TGIF!");
      } else {
        println!("まだ{:?}か", weekday);
      }
    }

    say_something(Weekday::Friday);
    say_something(Weekday::Monday);

    // データを持たない列挙型ではisizeを割り当てられる
    #[derive(Debug, PartialEq)]
    enum Month {
      January = 1,
      February = 2,
      March = 3,
    }
    assert_eq!(3, Month::March as isize);
    println!("Month = {:?} = {}月", Month::March, Month::March as isize);

    // データを持つ列挙型
    type UserName = String;

    #[derive(Debug)]
    enum Task {
      Open,
      AssignedTo(UserName),
      Working {
        assignee: UserName,
        remaining_hours: u16,
      },
      Done,
    }

    // // バリアント名を直接書くためにuse宣言をする
    // use crate::Task::*;

    let tasks = vec![
      Task::AssignedTo(String::from("junko")),
      Task::Working {
        assignee: String::from("hiro"),
        remaining_hours: 18,
      },
      Task::Done,
    ];

    for (i, task) in tasks.iter().enumerate() {
      match task {
        Task::AssignedTo(assignee) => {
          println!("タスク{}は{}さんにアサインされています", i, assignee)
        }
        Task::Working {
          assignee,
          remaining_hours,
        } => println!(
          "タスク{}は{}さんが作業中で、残り{}時間です",
          i, assignee, remaining_hours
        ),
        _ => println!("タスク{}はその他のステータス{:?}です", i, task),
      }
    }

    // enumもDefaultを実装できるが制約がある
    // いくつバリアントがあってもデフォルトは一つだけ
    // 構造体のような関数型レコードアップデート構文は使えない
  }

  #[test]
  fn type_detail() {
    // 構造体や列挙型は基本private
    // 公開する場合はpubを付ける
    // fieldにもそれぞれ指定が可能でprivate field二は外からアクセスできない
    mod shape {
      #[derive(Debug, Default)]
      pub struct Polygon {
        pub vertexts: Vec<(i32, i32)>,
        pub stroke_width: u8,
        pub fill: (u8, u8, u8),
        internal_id: String,
      }
      // 列挙型はそれ自体を公開するかどうかだけが選べる
      pub enum Ciel {
        T1,
        T2,
        T3 { height: u32, depth: u8 },
      }
      pub fn new(vertexts: Vec<(i32, i32)>) -> Polygon {
        Polygon {
          vertexts,
          ..Default::default()
        }
      }
    }
    let polygon = shape::new(vec![(0, 0), (1, 0), (2, 2)]);

    println!("{:?}", polygon);

    // 参照をもつ場合はライフタイム指定子を付ける
    struct StrRef<'a> {
      s1: &'a str,
      s2: &'a str,
    }

    #[derive(Debug, Default)]
    pub struct Polygon<T> {
      pub vertexes: Vec<T>,
    }
    trait Coordinates {}

    // デカルト座標
    #[derive(Default)]
    struct CartesianCoord {
      x: f64,
      y: f64,
    }
    impl Coordinates for CartesianCoord {}

    // 極座標
    #[derive(Default)]
    struct PolarCoord {
      r: f64,
      theta: f64,
    }
    impl Coordinates for PolarCoord {}

    let vertexes = vec![
      CartesianCoord { x: 0.0, y: 0.0 },
      CartesianCoord { x: 1.0, y: 2.0 },
    ];

    // CoordinatesのPolygon
    let poly = Polygon {
      vertexes,
      ..Default::default()
    };

    // 内部構造はprivateになっている。
    // 内部構造を暗黙の前提とした設計をさせないために。
    // しかしCとFFIやり取りするために構造体をC向けに合わせることができる
    // #[repr(C)]アトリビュートを付ける
  }

  #[test]
  fn type_cast() {
    let i1 = 42; //i32
    let f1 = i1 as f64 / 2.5; // f64
    let c1 = 'a';
    assert_eq!(97, c1 as u32);

    let i2 = 300;
    let u1 = i2 as u8;
    assert_eq!(44, u1); // 型あふれしている

    // can not complex type cast
    let t1 = ('a', 42);
    // let t2 = t1 as (u32, u8); // error
    let v1 = vec![b'h', b'e', b'l', b'l', b'o'];
    // let v2 = v1 as Vec<u16> // error

    // ひとつづつ型変換する
    let t3 = (t1.0 as u32, t1.1 as u8);
    let v3 = v1.iter().map(|&n| n as u16).collect::<Vec<u16>>();

    // 標準ライブラリには複合型を変換するためのFromトレイトが用意されている
    let v4: Vec<u8> = From::from("hello");
    assert_eq!(v1, v4);
  }

  #[test]
  fn type_transmute() {
    // アンセーフな型変換の仕組みでコンパイラのイントリンシック(組み込み関数)として表現されている。メモリ上のサイズが同じならどんな方でも変換できる

    // Box<i32>
    let p1 = Box::new(10);
    // let p2 = p1 as *mut i32; // 通常これはできない
    let p3: *mut i32 = unsafe { std::mem::transmute(p1) };

    // 正確には型変換とは違う
    // 以下の例をみよ
    let f1 = 5.6789e+3_f32;
    let i1 = f1 as u32;
    assert_eq!(i1, 5678);

    let i2: i32 = unsafe { std::mem::transmute(f1) };
    assert_eq!(i2, 1169258291);
  }

  #[test]
  fn type_coercion() {
    // 型強制はコンパイラが必要に応じて行う暗黙的な型変換
    // 簡潔に書ける代わりに餡目的なのでふるまいを把握しづらい

    // 3,4,5は通常u32だが、u8に変換されている
    let v1: Vec<u8> = vec![3, 4, 5];
    // もし型強制なしで表現すると次のようになる
    // let v1 = vec![3u8,4u8,5u8];

    // Vec<u8>から&[u8]に型強制されているため
    // first()が使える
    assert_eq!(Some(&3u8), v1.first());
    // 型強制がなかったら
    // assert_eq!(Some(&3u8), (v1[..]).first());

    let mut s1 = String::from("Type coercion ");
    let s2 = String::from("is actualy easy.");

    // 型強制によって&s2は&Stringから&strに変換
    s1.push_str(&s2);
    // 型強制が無かったら
    // (&myt s1).push_str(s2.as_str());

    // 変数定義、引数、任意の型
    // 型強制は再束縛でも起こる
    // 起こることがある場所、条件があることは知っておく

    // Derefによる型強制
    fn f1(n: &mut usize, s1: &str, slice: &[i32]) {
      *n = s1.len() + slice.len()
    }

    let mut b1 = Box::new(0);
    let s1 = String::from("deref");
    let v1 = vec![1, 2, 3];

    // Derefによる型強制
    // &mut Box<usize> -> &mut usize
    // &String -> &str
    // &Vec<i32> -> &[i32]
    f1(&mut b1, &s1, &v1);
    assert_eq!(8, *b1);

    // pointer weakening
    // あるポインタがより機能が制限された別のポインタに強制できるもの
    // mutabilityの除去 &mut T -> &T
    // 生ポインターへの変換 &T -> *const T
    fn f2(slice: &[usize]) -> usize {
      slice.len()
    }
    fn f3(slice: &mut [usize]) {
      let len = f2(slice);
      slice[0] = len;
    }
    let mut v = vec![0; 10];
    f3(&mut v[..]);
    assert_eq!(10, v[0]);

    // unsizing
    // 配列への参照をスライスへ変換できる &[T;N] -> &[T]
    fn f4(p: &[i32]) -> i32 {
      p[0]
    }
    fn f5(p: Box<[i32]>) -> i32 {
      p[0]
    }
    let a1 = [1, 2, 3, 4];
    assert_eq!(1, f4(&a1));
    assert_eq!(1, f5(Box::new(a1)));

    // method receiver coercion
    // Vec<u8>にはfirst()は実装されていないので
    // 1. selfのT型にメソッドがあるか
    // 2. T型のトレイトメソッドがあるか
    // 3. &T型にメソッドがあるか
    // 4. &T型にトレイトメソッドがあるか
    // 5. &mut Tにメソッドがあるか
    // の順でレシーバーを探して型強制を行う
    let v1: Vec<u8> = vec![3, 4, 5];
    assert_eq!(v1.first(), Some(&3u8));
  }
}
