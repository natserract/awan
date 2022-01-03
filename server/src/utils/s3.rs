use crate::types::Result as ResultT;
use dotenv::dotenv;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use s3::serde_types::ListBucketResult;
use s3::Region;
use std::env;
use std::process::{Command, Output};

#[derive(Debug)]
pub struct S3Config {
  pub region: String,
  pub bucket: String,
  access_key_id: String,
  secret_access_key: String,
  pub credentials: Credentials,
}

pub fn s3_config() -> S3Config {
  dotenv().ok();

  // Get required aws environment
  let aws_region = env::var("AWS_REGION").expect("Environment AWS_REGION not found");
  let aws_bucket = env::var("AWS_BUCKET").expect("Environment AWS_BUCKET not found");
  let aws_access_key =
    env::var("AWS_ACCESS_KEY_ID").expect("Environment AWS_ACCESS_KEY_ID not found");
  let aws_secret_key =
    env::var("AWS_SECRET_ACCESS_KEY").expect("Environment AWS_SECRET_ACCESS_KEY not found");

  // Set credentials directly from environment
  let aws_credentials = Credentials::new(
    Some(&aws_access_key),
    Some(&aws_secret_key),
    None,
    None,
    None,
  )
  .expect("Failed to read credentials");

  let aws_config = S3Config {
    region: aws_region,
    bucket: aws_bucket,
    access_key_id: aws_access_key,
    secret_access_key: aws_secret_key,
    credentials: aws_credentials,
  };

  aws_config
}

pub fn get_s3_bucket() -> ResultT<Bucket> {
  let config = s3_config();
  let bucket_name = config.bucket.as_str();
  let credentials: Credentials = config.credentials.clone();
  let region: Region = config.region.parse().unwrap();

  let bucket = Bucket::new(bucket_name, region, credentials).unwrap();

  Ok(bucket)
}

pub async fn list_s3_objects(
  bucket: Bucket,
  prefix: Option<String>,
) -> ResultT<Vec<ListBucketResult>> {
  let prefix_o = 
    if let Some(p) = prefix { p } else {
      "".to_string()
    };

  // @see: https://rust-lang.github.io/rust-clippy/master/#single_match_else
  // let prefix_o = match prefix {
  //   Some(p) => p,
  //   None => "".to_string(),
  // };

  bucket.list(prefix_o, Some("".to_string())).await
}

pub fn get_presigned_url(bucket: Bucket, key: &str) -> String {
  let expire_secs: u32 = 100;
  let presigned_url = bucket.presign_get(key, expire_secs);

  match presigned_url {
    Ok(p) => p,
    Err(error) => panic!("Error can't get presigned url {:?}", error),
  }
}

pub async fn delete_s3_object(bucket: Bucket, key: &str) -> String {
  let delete_obj = bucket.delete_object(key).await;

  match delete_obj {
    Ok(_) => "Object succesfully delete".to_string(),
    Err(error) => panic!("Error can't deleted object {:?}", error),
  }
}

pub fn command_list_s3_objects() -> Output {
  let config = s3_config();
  let layer = format!("s3://{}/", config.bucket);
  let run_aws_cli = Command::new("aws")
    .args(["s3", "ls", layer.as_str()])
    .output()
    .expect("Failed to execute aws s3 command");

  run_aws_cli
}
