use std::convert::TryFrom;
use std::{env, fs};

fn read_file(filename: &str) -> String {
    fs::read_to_string(filename).unwrap()
}

fn add_two_vecs(first: &[u32], second: &[u32]) -> Vec<u32> {
    first
        .iter()
        .zip(second.iter())
        .map(|(a, b)| a + b)
        .collect()
}

fn get_gamma_vec(values: &[Vec<u32>]) -> Vec<u32> {
    let threshold = u32::try_from(values.len()).unwrap() / 2;
    let occurrences: Vec<u32> = values
        .iter()
        .fold(vec![0; 12], |total, vals| add_two_vecs(&total, vals));
    let gamma_vec: Vec<u32> = occurrences
        .iter()
        .map(|digit| (digit > &threshold) as u32)
        .collect();
    gamma_vec
}

fn get_epsilon_vec(values: &[Vec<u32>]) -> Vec<u32> {
    let threshold = u32::try_from(values.len()).unwrap() / 2;
    let occurrences: Vec<u32> = values
        .iter()
        .fold(vec![0; 12], |total, vals| add_two_vecs(&total, vals));
    let gamma_vec: Vec<u32> = occurrences
        .iter()
        .map(|digit| (digit <= &threshold) as u32)
        .collect();
    gamma_vec
}

fn convert_to_int(vec: &[u32]) -> u32 {
    let init: u32 = 0;
    let base: u32 = 2;
    vec.iter()
        .rev()
        .zip(0..)
        .fold(init, |total, (val, index)| total + val * base.pow(index))
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = read_file(filename);

    let base: u32 = 2;

    let values: Vec<Vec<u32>> = contents
        .split('\n')
        .map(|line| line.chars().map(|d| d.to_digit(base).unwrap()).collect())
        .collect();

    let gamma_vec = get_gamma_vec(&values);
    let epsilon_vec = get_epsilon_vec(&values);

    let gamma = convert_to_int(&gamma_vec);
    let epsilon = convert_to_int(&epsilon_vec);

    println!("Part 1: {:?}", gamma * epsilon);
}
