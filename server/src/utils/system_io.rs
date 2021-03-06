use std::fs::File;
use std::io::{BufRead, BufReader};
use std::{env, io, path, process};

const EXIT_KEY: &str = "k\n";
const EXIT_CODE: i32 = 0x0100;

pub fn _exit_by(n: String) -> () {
    let is_exit = n == EXIT_KEY;

    if is_exit {
        process::exit(EXIT_CODE)
    };
}

pub fn read_lines() -> Vec<String> {
    let mut vec = Vec::new();
    let mut reader_input = String::new();

    let stdin = io::stdin();

    let match_exit = |n: String, clocur: fn()| -> () {
        let is_exit = n == EXIT_KEY;

        if is_exit {
            println!("Bye bye..");
            clocur();
        };
    };

    while let Ok(len) = stdin.read_line(&mut reader_input) {
        match len > 0 {
            true => {
                let input = reader_input.to_owned();

                match_exit(input, || loop {
                    process::exit(EXIT_CODE);
                });

                vec.push(reader_input);
                reader_input = String::new();
            }
            false => {
                panic!("Failed to read line!");
            }
        }
    }

    vec
}

pub fn read_file(file_path: &str) -> Vec<String> {
    let path = path::Path::new(file_path);
    let file = File::open(path).unwrap();

    let buf = BufReader::new(file)
        .lines()
        .map(|x| x.unwrap())
        .collect::<Vec<String>>();

    buf
}

pub fn read_args() -> Vec<String> {
    let args: Vec<String> = env::args().collect();
    let mut vect = Vec::new();

    for item in args.iter().enumerate() {
        let (i, val): (usize, &String) = item;
        let start_offset = i > 0;

        if start_offset {
            vect.push(val.to_string());
        }
    }

    vect
}
