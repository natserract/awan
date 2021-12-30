// [Rust] about references
//
// Immutable references &T, which allow sharing but not mutation.
// There can be multiple &T references to the same value simultaneously,
// but the value cannot be mutated while those references are active.
//
// Mutable references &mut T, which allow mutation but not sharing.
// If there is an &mut T reference to a value, there can be no other active references
// at that time, but the value can be mutated.
//
use super::types::Result as ResultT;
use super::{
    s3::{get_presigned_url, get_s3_bucket, list_s3_contents, list_s3_objects},
    system_io,
};
use system_io::{read_args, read_file, read_lines};

pub async fn run() -> ResultT<()> {
    let args: Vec<String> = read_args();
    println!("args {:?}", args);

    let readfile_lines = read_file("data.json");
    println!("Read: {:?}", readfile_lines);

    // AWS S3
    let s3_bucket = &get_s3_bucket().unwrap();
    let s3_contents = list_s3_contents(s3_bucket.to_owned(), None).await?;
    let presigned_url = get_presigned_url(s3_bucket.to_owned(), "zs8Nu5kV4XJ9EaTo3t6Sn788crgdioUYKC1HURzh.png");
    let s3_list_objects = list_s3_objects();

    println!("See list s3: {:?}", s3_contents);
    println!("See list s3: {:?}", presigned_url);

    let readlines = read_lines();
    println!("{:?}", readlines);

    Ok(())
}
