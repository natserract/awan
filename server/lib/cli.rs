use crate::lib::{s3, system_io};
use system_io::{read_args, read_file, read_lines};

pub fn run() {
    let args: Vec<String> = read_args();
    println!("args {:?}", args);

    let readfile_lines = read_file("data.json");
    println!("Read: {:?}", readfile_lines);
    
    // S3 test
    s3::list_s3_objects();

    let readlines = read_lines();
    println!("{:?}", readlines);

}
