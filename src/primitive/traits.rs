use std::str::FromStr;
// 関連型の定義
trait Server {
  type Response;
  type Request: FromStr;
  fn handle(&self, req: Self::Request) -> Self::Response;
}
struct EchoServer;
impl Server for EchoServer {
  type Response = String;
  type Request = String;
  fn handle(&self, req: Self::Request) -> Self::Response {
    req
  }
}

// S::Responseのように関連型を指定する
fn handle<S: Server<Request = String>>(server: S, req: &str) -> S::Response {
  server.handle(req.to_string())
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn associated_type() {
    let s = EchoServer {};
    let res = handle(s, "hello");
    assert_eq!("hello", res);
  }
}
