use std::{env, fs};


fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = read_file(filename);

}