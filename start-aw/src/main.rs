use actix_web::{fs, server, App, Error, FromRequest, HttpRequest, Path, Responder, State};
use serde_derive::*;

#[derive(Deserialize)]
struct HelloPath {
    // {name}に対するフィールドを定義
    name: String,
}

struct MyApp {
    server_name: String,
}

fn hello(req: &HttpRequest) -> impl Responder {
    let to = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", to)
}

fn hello_name(to: Path<HelloPath>) -> impl Responder {
    format!("Hello {}!", to.name)
}

fn hello_with_state(app: State<MyApp>) -> impl Responder {
    format!("Hello from {}!", app.server_name)
}

fn main() {
    server::new(|| {
        App::new()
            .resource("/", |r| r.f(hello))
            .resource("/{name}", |r| r.with(hello_name))
            .handler("/static", fs::StaticFiles::new(".").unwrap())
        // with_stateもできるがこの場合はhandlerの型が req:&HttpRequest<MyApp>になって汎用性がなくなる
        // App::with_state(MyApp {
        //     server_name: "server with state".into(),
        // })
        // .resource("/info", |r| r.with(hello_with_state))
    })
    .bind("localhost:3000")
    .expect("Can not bind to port 3000")
    .run();
}
