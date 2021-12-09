fn calculate_fish(timer: usize, days: usize, memo: &mut [Vec<usize>]) -> usize {
    if memo[timer][days] != 0 {
        memo[timer][days]
    } else if timer >= days {
        1
    } else {
        let current_fish = calculate_fish(6, days - timer - 1, memo);
        let offspring = calculate_fish(8, days - timer - 1, memo);
        memo[timer][days] = current_fish + offspring;
        current_fish + offspring
    }
}

fn main() {
    let contents = include_str!("day6.txt");

    let mut memo: Vec<Vec<usize>> = vec![vec![0; 257]; 9];

    let fish: Vec<usize> = contents
        .split(',')
        .map(|ele| ele.parse::<usize>().unwrap())
        .collect();

    let p1_total: usize = fish
        .iter()
        .map(|&timer| calculate_fish(timer, 80, &mut memo))
        .sum();

    println!("Part 1: {:?}", p1_total);

    let p2_total: usize = fish
        .iter()
        .map(|&timer| calculate_fish(timer, 256, &mut memo))
        .sum();

    println!("Part 2: {:?}", p2_total);
}
