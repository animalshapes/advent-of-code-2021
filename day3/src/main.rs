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

fn filter_by_occurrence(
    values: Vec<Vec<u32>>,
    index: usize,
    comparison: &dyn Fn(i32, i32) -> bool,
) -> Vec<Vec<u32>> {
    let counts = values.iter().fold((0, 0), |count, vec| {
        let val = &vec[index];
        match val {
            0 => (count.0 + 1, count.1),
            1 => (count.0, count.1 + 1),
            _ => panic!("unexpected input"),
        }
    });
    let keep = comparison(counts.0, counts.1) as u32;
    values
        .into_iter()
        .filter(|vec| vec[index] == keep)
        .collect::<Vec<_>>()
}

fn get_gamma_vec(values: &[Vec<u32>]) -> Vec<u32> {
    let threshold = u32::try_from(values.len()).unwrap() / 2;
    let occurrences: Vec<u32> = values
        .iter()
        .fold(vec![0; 12], |total, vals| add_two_vecs(&total, vals));
    occurrences
        .iter()
        .map(|digit| (digit > &threshold) as u32)
        .collect::<Vec<_>>()
}

fn get_epsilon_vec(values: &[Vec<u32>]) -> Vec<u32> {
    let threshold = u32::try_from(values.len()).unwrap() / 2;
    let occurrences: Vec<u32> = values
        .iter()
        .fold(vec![0; 12], |total, vals| add_two_vecs(&total, vals));
    occurrences
        .iter()
        .map(|digit| (digit <= &threshold) as u32)
        .collect::<Vec<_>>()
}

fn get_oxygen_vec(values: &[Vec<u32>]) -> Vec<u32> {
    let len: usize = 12;
    let mut new_values = values.to_vec();
    for i in 0..len {
        new_values = filter_by_occurrence(new_values, i, &|a, b| a <= b);
        if new_values.len() == 1 { break }
    }
    new_values.first().unwrap().to_vec()
}

fn get_co2_vec(values: &[Vec<u32>]) -> Vec<u32> {
    let len: usize = 12;
    let mut new_values = values.to_vec();
    for i in 0..len {
        new_values = filter_by_occurrence(new_values, i, &|a, b| a > b);
        if new_values.len() == 1 { break }
    }
    new_values.first().unwrap().to_vec()
}

fn convert_to_int(vec: &[u32], base: u32) -> u32 {
    let init: u32 = 0;
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

    let gamma = convert_to_int(&gamma_vec, base);
    let epsilon = convert_to_int(&epsilon_vec, base);

    println!("Part 1: {:?}", gamma * epsilon);

    let oxygen_vec = get_oxygen_vec(&values);
    let co2_vec = get_co2_vec(&values);

    let oxygen = convert_to_int(&oxygen_vec, base);
    let co2 = convert_to_int(&co2_vec, base);

    println!("Part 2: {:?}", oxygen * co2);
}
