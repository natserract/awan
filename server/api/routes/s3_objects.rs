use actix_web::{get, web, HttpResponse, Responder};
use serde_json::to_string_pretty as json;

#[get("/")]
pub async fn index() -> impl Responder {
  // @see https://stackoverflow.com/questions/66130788/return-either-html-or-json-from-actix-web-handler
  HttpResponse::Ok().body(
    json("Hello").unwrap()
  ).await
}
