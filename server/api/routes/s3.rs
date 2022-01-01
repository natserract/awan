use crate::utils::s3::{
  get_presigned_url as presigned_url_func, get_s3_bucket, list_s3_objects,
};
use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use chrono::{DateTime, Utc};

use crate::types::{ErrorE, Result as ResultT};
use s3::serde_types::{ListBucketResult, Object as ObjectT};
use serde::ser::{SerializeStruct, Serializer};
use serde::{Deserialize, Serialize};
use serde_json::{self, json, to_string_pretty as json_pretty};
use std::{ptr, rc::Rc};
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

pub struct ListObjectsResponses {
  objects: Vec<ObjectT>,
}

impl Serialize for ListObjectsResponses {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    // let value = Rc::from(&self.objects);
    // let values = vec![value];

    // let j = json!(value);
    // let to_json = serde_json::from_value::<Vec<Rc<ListBucketResult>>>(j).unwrap();

    let mut state = serializer.serialize_struct("ListObjectsResponses", 1)?;
    state.serialize_field("objects", &self)?;
    state.end()

    // for t in value {
    //   sq.serialize_element(t.as_ref())?;
    // }

    // sq.end()
  }
}

pub async fn get_list_objects() -> impl Responder {
  let s3_bucket = get_s3_bucket().unwrap();
  let results = runtime().block_on(async {
    list_s3_objects(s3_bucket, None).await
  }).unwrap();

  HttpResponse::Ok()
    .body(json!(results))
    .await
}

pub async fn get_list_objects2() -> impl Responder {
  HttpResponse::Ok()
    .content_type("application/json")
    .body(json_pretty("hello").unwrap())
    .await
}
