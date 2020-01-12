use actix_web::http::Method;
use actix_web::App;

mod handlers;
mod model;
mod schema;

#[macro_use]
extern crate diesel;

#[derive(Clone)]
pub struct Server {}

impl Server {
    pub fn new() -> Server {
        Server {}
    }
}

pub fn app(server: Server) -> App<Server> {
    use crate::handlers::*;

    let app: App<Server> = App::with_state(server)
        .route("/logs", Method::POST, handler_post_logs)
        .route("/csv", Method::POST, handler_post_csv)
        .route("/logs", Method::GET, handler_get_logs)
        .route("/csv", Method::GET, handler_get_csv);
    app
}

fn main() {
    env_logger::init();
    let server = Server::new();
    actix_web::server::new(move || app(server.clone()))
        .bind("localhost:3000")
        .expect("Can not bind to port 3000")
        .run();
}
