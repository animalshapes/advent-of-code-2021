use std::collections::HashMap;

fn move_steps(pos: u64, steps: u64) -> u64 {
    (pos + steps - 1) % 10 + 1
}

fn deterministic_dice(p1: u64, p2: u64) -> u64 {
    let mut current_roll = 1;
    let mut p1_pos = p1;
    let mut p2_pos = p2;
    let mut p1_score = 0u64;
    let mut p2_score = 0u64;

    let mut p1_turn = true;

    while p1_score < 1000 && p2_score < 1000 {
        let total_steps = 3 * current_roll + 3;
        if p1_turn {
            p1_pos = move_steps(p1_pos, total_steps);
            p1_score += p1_pos;
        } else {
            p2_pos = move_steps(p2_pos, total_steps);
            p2_score += p2_pos;
        }
        p1_turn = !p1_turn;
        current_roll += 3
    }

    p1_score.min(p2_score) * (current_roll - 1)
}

fn calculate_outcomes(
    p1_pos: u64,
    p2_pos: u64,
    p1_score: u64,
    p2_score: u64,
    cache: &mut HashMap<(u64, u64, u64, u64), (u64, u64)>,
) -> (u64, u64) {
    if p2_score >= 21 {
        return (0, 1);
    }
    if let Some(&score) = cache.get(&(p1_pos, p2_pos, p1_score, p2_score)) {
        return score;
    }

    let mut score = (0u64, 0u64);
    for (steps, multiplier) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        let new_pos = move_steps(p1_pos, steps);
        let (p1_to_add, p2_to_add) =
            calculate_outcomes(p2_pos, new_pos, p2_score, p1_score + new_pos, cache);
        score = (
            score.0 + multiplier * p2_to_add,
            score.1 + multiplier * p1_to_add,
        )
    }

    cache.insert((p1_pos, p2_pos, p1_score, p2_score), score);
    score
}

fn dirac_dice(p1_pos: u64, p2_pos: u64) -> u64 {
    let mut cache: HashMap<(u64, u64, u64, u64), (u64, u64)> = HashMap::new();
    let (p1_winner, p2_winner) = calculate_outcomes(p1_pos, p2_pos, 0, 0, &mut cache);
    p1_winner.max(p2_winner)
}

fn main() {
    let contents = include_str!("day21.txt").trim_end();

    let positions: Vec<u64> = contents
        .lines()
        .map(|row| u64::from((row.bytes().last().unwrap() as char).to_digit(10).unwrap()))
        .collect();

    let part_1 = deterministic_dice(positions[0], positions[1]);
    println!("Part 1: {:?}", part_1);

    let part_2 = dirac_dice(positions[0], positions[1]);
    println!("Part 2: {:?}", part_2);
}
