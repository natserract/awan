use dotenv::dotenv;
use s3::bucket::Bucket;
use s3::creds::Credentials;
use std::env;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T, E = Error> = std::result::Result<T, E>;
#[derive(Debug)]
pub struct S3Config {
  region: String,
  bucket: String,
  access_key_id: String,
  secret_access_key: String,
  credentials: Credentials,
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
  ).expect("Failed to read credentials");

  let aws_config = S3Config {
    region: aws_region,
    bucket: aws_bucket,
    access_key_id: aws_access_key,
    secret_access_key: aws_secret_key,
    credentials: aws_credentials,
  };

  aws_config
}


pub async fn list_s3_objects() -> Result<()> {
  println!("Test");

  let config = s3_config();
  let bucket_name = config.bucket.as_str();
  let credentials = config.credentials.clone();
  let region = config.region.parse().unwrap();

  let bucket = Bucket::new(bucket_name, region, credentials).unwrap();

  let results = 
    bucket.list(
      "/".to_string(), 
      Some("".to_string())
    ).await?;
  // let results = bucket.get_object("/").await?;
  // let location = bucket.location().await?;
  // let c = bucket.
  
  println!("Region {:?}", results);

  Ok(())
}
