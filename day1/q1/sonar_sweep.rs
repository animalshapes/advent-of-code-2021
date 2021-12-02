use std::env;
use std::fs;

fn main() {
    let filename = "input.txt";

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}