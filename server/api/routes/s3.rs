use crate::utils::s3::{
  get_presigned_url as presigned_url_func, get_s3_bucket, list_s3_objects,
};
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use chrono::{DateTime, Utc};

use s3::serde_types::{Object as S3Object};
use serde::{Deserialize, Serialize};
use serde_json::{self, json, to_string_pretty as json_pretty};

use web::Query as RequestQuery;
use crate::runtime;

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
  let responses = PresignedUrlResponses {
    message: "Successfully get presigned url".to_string(),
    presigned_url,
    created_at: chrono::offset::Utc::now(),
  };

  HttpResponse::Ok()
    .content_type("application/json")
    .body(json_pretty(&responses).unwrap())
    .await
}

#[derive(Debug, Serialize)]
struct FieldObjects {
  id: usize,
  key: String,
  last_modified: String,
  size: u64,
}

#[derive(Serialize)]
pub struct ListObjectsResponses {
  objects: Vec<FieldObjects>
}

pub async fn get_list_objects() -> impl Responder {
  let s3_bucket = get_s3_bucket().unwrap();
  let responses = 
    runtime()
      .block_on(async {
        list_s3_objects(s3_bucket, None).await
      })
    .unwrap();

  let responses_it = 
  responses.iter().cloned().map(|p| p.contents).into_iter();

  let mut objects: Vec<FieldObjects> = Vec::new();

  for item in responses_it.enumerate() {
    let (_, lists): (usize, Vec<S3Object>) = item.clone();
    
    lists.iter().enumerate().for_each(|(i, o)| {
      objects.push(FieldObjects {
        id: i,
        key: o.key.clone(),
        last_modified: o.last_modified.clone(),
        size: o.size.clone(),
      })
    });
  }

  HttpResponse::Ok()
    .body(json!(
      ListObjectsResponses {
        objects,
      }
    ))
    .await
}

pub async fn get_list_objects2() -> impl Responder {
  HttpResponse::Ok()
    .content_type("application/json")
    .body(json_pretty("hello").unwrap())
    .await
}
