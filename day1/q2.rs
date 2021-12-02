use std::fs;

fn main() {
    let filename = "input.txt";

    let contents = fs::read_to_string(filename).expect("Something went wrong reading the file");

    let numbers: Vec<i32> = contents
        .split("\n")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    let windows: Vec<i32> = numbers.windows(3).map(|w| w.iter().sum()).collect();

    let increasing: Vec<i32> = windows
        .iter()
        .zip(windows.iter().skip(1))
        .map(|(a, b)| (b > a) as i32)
        .collect();

    let sum: i32 = increasing.iter().sum();

    println!("{}", sum);
}
