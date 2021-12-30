/// [Rust] about references
///
/// Immutable references &T, which allow sharing but not mutation. 
/// There can be multiple &T references to the same value simultaneously, 
/// but the value cannot be mutated while those references are active.
///
/// Mutable references &mut T, which allow mutation but not sharing. 
/// If there is an &mut T reference to a value, there can be no other active references 
/// at that time, but the value can be mutated.
///

use super::s3::list_s3_objects;
use super::system_io;
use super::types::Result as ResultT;
use system_io::{read_args, read_file, read_lines};

pub async fn run() -> ResultT<()> {
    // let args: Vec<String> = read_args();
    // println!("args {:?}", args);

    // let readfile_lines = read_file("data.json");
    // println!("Read: {:?}", readfile_lines);
    // S3 test
    list_s3_objects().await?;

    // let readlines = read_lines();
    // println!("{:?}", readlines);

    Ok(())
}
