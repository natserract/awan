use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::to_string_pretty as json;
use web::Query as RequestQuery;

#[get("/")]
pub async fn index() -> impl Responder {
  HttpResponse::Ok()
    .body("Index")
    .await
}

#[derive(Debug, Deserialize)]
pub struct PresignedUrlQueryParams {
  filekey: String,
}

pub async fn get_presigned_url(
  query_params: RequestQuery<PresignedUrlQueryParams>,
  request: HttpRequest,
) -> impl Responder {
  println!("QUERY PARAMS {:?}", query_params);
  println!("REQ: {:?}", request);

  // @see https://stackoverflow.com/questions/66130788/return-either-html-or-json-from-actix-web-handler
  HttpResponse::Ok()
    .body(json("GET PRESIGNED URL").unwrap())
    .await
}
