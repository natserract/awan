use std::process;
use std::{borrow::BorrowMut, io};

const EXIT_KEY: &str = "k\n";

pub fn read_lines() -> Vec<String> {
    println!("Welcome! \n");
    println!("Please type k, for exit! \n");

    let mut vec = Vec::new();
    let mut reader_input = String::new();

    let stdin = io::stdin();

    let match_exit = |n: String, closure: fn()| -> () {
        let is_exit = n == EXIT_KEY;

        if is_exit {
            println!("Bye bye..!");
            closure();
        };
    };

    while let Ok(len) = stdin.read_line(&mut reader_input) {
        match len > 0 {
            true => {
                let input = reader_input.to_owned();

                // Exit key if user type `k`
                match_exit(input, || process::exit(0x0100));

                vec.push(reader_input);
                reader_input = String::new();
            }
            false => {
                panic!("Failed to read line!");
                break;
            }
        }
    }

    vec
}
