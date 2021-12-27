use dotenv::dotenv;
use std::env;
use std::io;
use std::io::Write;

use crate::lib::helper;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let readlines = helper::read_lines();

    println!("{:?}", readlines);

    for item in args.iter().enumerate() {
        let (i, v): (usize, &String) = item;
        let start_offset = i > 0;

        if start_offset {
            println!("Index: {}, Collect {:?}", i, v);
        }
    }
}
