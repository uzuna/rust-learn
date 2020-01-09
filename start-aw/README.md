actix webを使って実装を試す

IOの比重が大きい場合は非同期によってCPUを効率よく使いながら、比較的容易に書くことができる。
動機では簡単に書けるが、待ち時間が長くなり、それを同期のまま回避可能に書くのは難しい
Hyperはfutures周りの実装が大きく変わるので今は避ける。
アクターモデルはここでは解説しない


### Actix-Webの構成要素
- HttpServer: HTTPを処理するサーバ、コネクションやSSL、ワーカー数を設定できる
- App: アプリケーションデータの保持やリクエストのルーティングを担当
- ハンドラ: リクエストを処理する中身
- エクストラクタ: リクエストからデータを抽出す津
- ミドルウェア: リクエスト処理する前後に何かしらの処理をするもの


```rs

// Sはユーザーデータ

// actix_web::dev::Handler
pub trait Handler<S>: 'static {
  type Result: Responder
  fn handler(&self, req: &HttpRequest<S>) -> Self::Result
}

// Fnに対しても実装されている
impl<F, R, S>Handler<S> for F 
where
  F: Fn(&HttpRequest<S>) -> R+'static,
  R: Respoder + 'static,
{

};

pub trait Responder{
  type Item: Into<AsyncResult<HttpResponse>>;
  type Error: Into<Error>;
  fn respond_to<S: 'static> (
    self,
    req: &HttpRequest<S>
  ) -> Result<Self::Item,Self::Error>;
}

// stringからResponseを構成している
impl Responder for String{
  type Item: HttpResponse;
}

```