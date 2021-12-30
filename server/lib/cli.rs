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
