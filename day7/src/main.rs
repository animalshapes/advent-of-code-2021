fn p2_fuel(crabs: &[i32], meeting: i32) -> i32 {
    crabs
        .iter()
        .map(|pos| {
            let diff = (*pos - meeting).abs();
            diff * (diff + 1) / 2
        })
        .sum()
}

fn main() {
    let contents = include_str!("day7.txt");

    let mut crabs: Vec<i32> = contents
        .split(',')
        .map(|ele| ele.parse::<i32>().unwrap())
        .collect();

    crabs.sort_unstable();
    let num_crabs = crabs.len();
    let median = crabs[num_crabs / 2];

    let p1_fuel: i32 = crabs.iter().map(|pos| (*pos - median).abs()).sum();

    println!("Part 1: {:?}", p1_fuel);

    let p2_fuel: i32 = (0..1000)
        .map(|meeting| p2_fuel(&crabs, meeting))
        .min()
        .unwrap();

    println!("Part 2: {:?}", p2_fuel);
}
