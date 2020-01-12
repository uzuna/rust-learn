use actix_web::{HttpResponse, Json, Query, State};
use failure::Error;
use log::debug;

use crate::Server;

/// POST /csv handeler
pub fn handler_post_csv(server: State<Server>) -> Result<HttpResponse, Error> {
  let logs = Default::default();
  Ok(HttpResponse::Ok().json(api::csv::post::Response(logs)))
}

/// POST /logs handeler
pub fn handler_post_logs(
  server: State<Server>,
  log: Json<api::logs::post::Request>,
) -> Result<HttpResponse, Error> {
  debug!("{:?}", log);
  Ok(HttpResponse::Accepted().finish())
}

/// Get /logs handeler
pub fn handler_get_logs(
  server: State<Server>,
  range: Json<api::logs::get::Query>,
) -> Result<HttpResponse, Error> {
  debug!("{:?}", range);
  let logs = Default::default();
  Ok(HttpResponse::Ok().json(api::logs::get::Response(logs)))
}

/// Get /csv
pub fn handler_get_csv(
  server: State<Server>,
  range: Json<api::csv::get::Query>,
) -> Result<HttpResponse, Error> {
  debug!("{:?}", range);
  let csv: Vec<u8> = vec![];
  Ok(
    HttpResponse::Ok()
      .header("Content-Type", "text/csv")
      .body(csv),
  )
}
