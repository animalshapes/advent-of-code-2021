use std::{env, fs};

fn count_increasing(input: &[i32]) -> i32 {
    let increasing: Vec<i32> = input
        .iter()
        .zip(input.iter().skip(1))
        .map(|(a, b)| (b > a) as i32)
        .collect();

    let sum: i32 = increasing.iter().sum();
    sum
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];

    let contents = fs::read_to_string(filename).unwrap();

    let numbers: Vec<i32> = contents
        .split('\n')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    let q1_ans = count_increasing(&numbers);
    println!("Part 1: {}", q1_ans);

    let windows: Vec<i32> = numbers.windows(3).map(|w| w.iter().sum()).collect();
    let q2_ans = count_increasing(&windows);
    println!("Part 2: {}", q2_ans);
}
