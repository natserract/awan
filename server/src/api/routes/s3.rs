use crate::utils::s3::{
  get_presigned_url as presigned_url_func, 
  get_s3_bucket, 
  list_s3_objects,
  delete_s3_object,
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
pub struct QueryParams {
  filekey: String,
}

#[derive(Serialize)]
pub struct PresignedUrlResponses {
  message: String,
  presigned_url: String,
  created_at: DateTime<Utc>,
}

// # Method: GET
// -> Request: `/s3/presigned?filekey=<filekey>`
// 
pub async fn get_presigned_url(
  query_params: RequestQuery<QueryParams>,
  _request: HttpRequest,
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

// # Method: GET
// -> Request: `/s3/objects`
// 
pub async fn get_list_objects() -> impl Responder {
  let s3_bucket = get_s3_bucket().unwrap();
  let responses = 
    runtime()
      .block_on(async {
        list_s3_objects(s3_bucket, None).await
      })
    .unwrap();

  let iters = 
    responses.iter().cloned().map(|p| p.contents).into_iter();
  let mut objects: Vec<FieldObjects> = Vec::new();

  for item in iters.enumerate() {
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
    .content_type("application/json")
    .body(json!(
      ListObjectsResponses {
        objects,
      }
    ))
    .await
}

#[derive(Serialize)]
pub struct DeleteObjectResponses {
  key: String,
  message: String,
}

// # Method: DELETE
// -> Request: `/s3/delete?filekey=<filekey>`
// 
pub async fn delete_object(
  query_params: RequestQuery<QueryParams>,
  _request: HttpRequest
) -> impl Responder {
  let filekey = &query_params.filekey;

  let s3_bucket = get_s3_bucket().unwrap();
  let responses = 
    runtime()
      .block_on(async {
        delete_s3_object(s3_bucket, filekey).await
      });

  HttpResponse::Ok()
    .content_type("application/json")
    .body(json!(
      DeleteObjectResponses {
        key: filekey.clone(),
        message: responses,
      }
    ))
    .await
}

#[derive(Serialize)]
pub struct GetBucketNameResponses {
  bucket: String,
  region: String,
}

// # Method: GET
// -> Request: `/s3/bucket`
// 
pub async fn get_bucket_name() -> impl Responder {
  let s3_bucket = get_s3_bucket().unwrap();

  HttpResponse::Ok()
    .content_type("application/json")
    .body(json!(
      GetBucketNameResponses {
        bucket: s3_bucket.name,
        region: s3_bucket.region.to_string(),
      }
    ))
    .await
}