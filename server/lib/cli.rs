use crate::lib::read_io;
use read_io::{read_args, read_file, read_lines};

pub fn run() {
    let args: Vec<String> = read_args();
    println!("args {:?}", args);

    let readfile_lines = read_file("data.json");
    println!("Read: {:?}", readfile_lines);

    let readlines = read_lines();
    println!("{:?}", readlines);
}
