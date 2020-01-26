#[macro_use]
extern crate diesel;
use actix_service::ServiceFactory;
use actix_web::body::Body;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::Error;
use actix_web::{web, App, HttpServer};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool};
use dotenv::dotenv;
use std::env;

mod db;
mod handlers;
mod model;
mod schema;

#[derive(Clone)]
pub struct Server {
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl Server {
    pub fn new() -> Self {
        dotenv().ok();
        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
        let manager = ConnectionManager::<PgConnection>::new(database_url);
        let pool = Pool::builder()
            .build(manager)
            .expect("Failed to create pool.");
        Server { pool }
    }
}

pub fn app(
    server: Server,
) -> App<
    impl ServiceFactory<
        Config = (),
        Request = ServiceRequest,
        Response = ServiceResponse<Body>,
        Error = Error,
        InitError = (),
    >,
    Body,
> {
    use crate::handlers::*;

    App::new()
        .data(server)
        .service(
            web::resource("logs")
                .route(web::post().to(handler_post_logs))
                .route(web::get().to(handler_get_logs)),
        )
        .service(
            web::resource("csv")
                .route(web::post().to(handler_post_csv))
                .route(web::get().to(handler_get_csv)),
        )
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let server = Server::new();
    HttpServer::new(move || app(server.clone()))
        .bind("localhost:3000")?
        .run()
        .await
}
