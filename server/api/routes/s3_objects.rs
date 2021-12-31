use crate::utils::s3::{get_presigned_url as presigned_url_func, get_s3_bucket};
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use serde::{Deserialize, Serialize};
use serde_json::to_string_pretty as json;
use web::Query as RequestQuery;
use chrono::{DateTime, Utc};

#[get("/")]
pub async fn index() -> impl Responder {
  HttpResponse::Ok().body("Index").await
}

#[derive(Debug, Deserialize)]
pub struct PresignedUrlQueryParams {
  filekey: String,
}

#[derive(Serialize)]
pub struct PresignedUrlResponses {
  message: String,
  presigned_url: String,
  created_at: DateTime<Utc>,
}

pub async fn get_presigned_url(
  query_params: RequestQuery<PresignedUrlQueryParams>,
  request: HttpRequest,
) -> impl Responder {
  let filekey = &query_params.filekey;
  let s3_bucket = get_s3_bucket().unwrap();
  let presigned_url = presigned_url_func(s3_bucket, filekey);

  HttpResponse::Ok()
    .content_type("application/json")
    .body(json(
      &PresignedUrlResponses {
        message: "Successfully get presigned url".to_string(),
        presigned_url,
        created_at: chrono::offset::Utc::now()
      }
    ).unwrap())
    .await
}
