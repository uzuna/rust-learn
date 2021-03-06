#[cfg(test)]
mod tests {
  use std::io;
  #[test]
  fn shakyo() {
    fn is_leap_year(year: u32) -> bool {
      year % 4 == 0 && !(year % 100 == 0 && year % 400 != 0)
    }

    // useを使ってモジュールを利用可能にする
    // モジュールの中に入る要素をアイテムという
    // io::stdout().flush().unwrap();
    // io::stdin().read_line(&mut year).unwrap();

    // 関数は本体を除いたものをシグネチャと呼ぶ
    // Rustは式指向言語なのでプログラム要素は基本的に値を返すものになる
    // 関数も()を戻しているがこの場合は戻り値を省略できる
    // 本当に何も返さない場合は次のようになる
    // 組込みシステムなどで割り込みを待つ時とかに使う
    fn end_function() -> ! {
      std::process::exit(0)
    }

    // 式と文
    // 関数は0個以上の分の集まりで、最後は式でもよい
    // 文とは()を返すプログアム要素で;終端。
    //   宣言文: アイテムを宣言、変数を導入するlet文
    //   式文: 式を;終端で閉じたもの
    // 式とは()以外を返すプログラム要素

    // マクロ
    // 名前の末尾が!になっているのがマクロの呼び出し
    // 引数を受け取り、コード片を返すもの。

    struct Circle {
      radius: u32,
    }
    impl Circle {
      // メソッド
      // 構造体の中に定義された関数
      // implの中にfnで定義をして第一引数が&self
      fn diameter(&self) -> u32 {
        self.radius * 2
      }

      // 関連関数
      // メソッドは構造体のインスタンスに関連付けられたもの
      // 関連関数は構造体そのものに関連付けられる
      fn small_circle() -> Circle {
        Circle { radius: 1 }
      }
    }

    let c1 = Circle::small_circle();
    assert_eq!(2, c1.diameter());
  }

  #[test]
  fn let_mutable() {
    // 束縛とmutability
    let _date_string = "2019-01-06";
    let _pi: f64 = 3.14;
    let not_initialized; // 後から入れるなら初期化しなくてもよい
    let (_a, _b) = (19, 79); // パターンによって宣言

    not_initialized = 8;

    let mut mutable_string = String::from("String");
    mutable_string = String::from("Hello"); // 別の文字列で束縛
    mutable_string.push_str(" world!"); // 文字列に対する変更操作

    // mutable_string = 2019; // 型が違うものは入れられない

    // スコープ
    let x = 20;
    {
      // ブロックによってスコープが分かれる
      let y = 10;
      assert_eq!(20, x);
      assert_eq!(10, y);
      // yはこのブロックから外には出ない
    }
    assert_eq!(20, x);

    // シャドーイングによって同じ名前の変数を宣言することはできる
    // ブロック内に影響をとどめることはできる
    {
      assert_eq!("Hello world!", &mutable_string);
      let mutable_string = 2020;
      assert_eq!(2020, mutable_string);
    }
    assert_eq!("Hello world!", &mutable_string);
    let mutable_string = 2019;
    assert_eq!(2019, mutable_string);

    // const
    // コンパイル時に値が確定され、定数参照している場所に埋め込まれる
    // static
    // constに似ているが使われるたびに参照される
    // mut宣言もできるが読み書きはunsafeブロックの中でなければならない
  }

  #[test]
  fn if_let_expression() {
    // 式(Expressions)と文(Statements)

    // if式なので戻り値がある
    let a = 3;
    let even_or_odd = if a % 2 == 0 { "even" } else { "odd" };

    // match式
    // 上から順にパターンマッチされて、該当するパターンが実行される。
    // 式なので戻り値が取れる
    let value = 100;
    let string = match value {
      1 => "One",
      10 => "Ten",
      100 => "Handred",

      // パターンは網羅されている必要があるため
      // _ でu32が取りうる値で上のパターン以外すべてをとる
      _ => "Something else",
    };
    assert_eq!("Handred", string);

    // enumの場合はすべての可能性を列挙できるので
    // _ がないパターンマッチもできる
    enum Light {
      Red,
      Yellow,
      Green,
    }

    let light = Light::Green;
    let action = match light {
      Light::Red => "Stop",
      Light::Yellow => "Proceed with caution",
      Light::Green => "Go",
    };
    assert_eq!("Go", action);

    // Pattern Optional
    // 複合型から中の値を取り出して使うことができる。
    // 分配束縛といわれるもので、列挙、構造体、タプル、参照について行うことができる
    let unknwon = Some("Apple");
    let string = match unknwon {
      Some(something) => String::from("Hi, ") + something,
      None => String::from("Nothing"),
    };
    assert_eq!(string, "Hi, Apple");

    // reference
    let ten = 10;
    let ten_ref = &ten;

    match ten_ref {
      number => assert_eq!(&10, number),
    };
    match ten_ref {
      &number => assert_eq!(10, number),
    };

    // 複数のパターン連結と範囲指定
    let number = 42;
    let string = match number {
      1 | 2 | 3 => "One or Two or Three",
      40...50 => "From 40 to 50",
      _ => "Somuthing else",
    };
    assert_eq!("From 40 to 50", string);

    // 条件付きパターン。条件のことをガードという
    let string = Some("This is a very long string");
    let message = match string {
      Some(s) if s.len() > 10 => "Long string",
      Some(_) => "String",
      None => "Nothing",
    };
    assert_eq!("Long string", message);

    // if let suggar syntax
    // matchの糖衣構文であるパターンにパッチした場合とそれ以外の分岐
    let score = Some(100);
    if let Some(100) = score {
      assert_eq!(Some(100), score)
    } else {
      unreachable!();
    }
  }

  #[test]
  fn syntax_loop() {
    // loop式なのでbreakの値を取ることができる
    // 一番内側から抜けるため、ラベルを付ける場合は '<label>: loop
    let mut counter = 0;
    let ten = 'outer: loop {
      if counter >= 10 {
        break 'outer counter;
      }
      counter += 1;
    };
    assert_eq!(10, counter);

    // whileは条件付きループ。
    // breakでは値が返せず常に()になる
    counter = 0;
    while counter >= 10 {
      counter += 1;
    }

    // while let
    // 条件を満たす場合はwhileの中を実行する
    let mut counter = Some(0);
    while let Some(i) = counter {
      if i == 10 {
        counter = None;
      } else {
        counter = Some(i + 1);
      }
    }

    // for
    // 要素をループする
    // break,continueも使えるが返る値は常に()
    let vector = vec!["Cyan", "Magenta", "Yellow", "Black"];
    for v in vector.iter() {
      println!("{}", v);
    }
  }

  #[test]
  fn closure() {
    let mut one = 1;
    let plus_one = |x| x + one;
    assert_eq!(11, plus_one(10));

    // oneをクロージャに貸しているためoneは変更できない
    // moveを使うと値をコピーするため
    // 値は変更できるようになる。束縛した値は無関係になる
    let plus_one = move |x| x + one;
    one += 1;
    assert_eq!(11, plus_one(10));
    assert_eq!(2, one);
  }

  #[test]
  fn attribute() {
    // アイテム宣言にメタデータを付けるもの
    // #[test]のようにアイテム宣言の前に書く方法と

    fn test1() {
      #![test]
      //アイテム宣言の中に書く方法がある
    }
    // test attributeは testをオプションを渡した時だけコンパイルされる
    // #[cfg(unix)] // ターゲットがunixの時だけ
    // #[cfg(windows)] // ターゲットがwindowsの時だけ
    // #[cfg(allfnc)] // コンパイル時に"--cfg alifns"を渡した時だけ
    // #[cfg(color = "blue")] // コンパイル時に"--cfg color="blue""を渡した時だけ
    // #[derive(Debug)]
    // Debug トレイトを自動的に実装する
    // #[allow(unused_imports)]
    // 使わないクレートのリントチェックを無視
    // #[deny(dead_code)]
  }

  #[test]
  fn mod_pub() {
    mod server {
      // pubを付けなければプライベートになる
      pub fn echo() -> String {
        String::from("echo!")
      }
    }
    mod crate_server {
      // server moduleの含まれるクレートに対してはpublic
      pub(crate) fn echo() -> String {
        String::from("echo!")
      }
    }
    // mod app_server {
    //   // app::network moduleではpublic
    //   pub(in app::network) fn echo() -> String {
    //     String::from("echo!")
    //   }
    // }
    assert_eq!("echo!", &server::echo());

    // fileに切り出した場合は
    // mod <filename>;をsrc/main.rsもしくは<dir>/mod.rs内で行う
  }
}
