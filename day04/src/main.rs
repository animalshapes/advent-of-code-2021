use std::collections::HashSet;

fn check_board(board_combo: &[HashSet<&u32>], draws: &HashSet<&u32>) -> bool {
    let num_matches = board_combo
        .iter()
        .filter(|&combo| combo.is_subset(draws))
        .count();
    num_matches > 0
}

fn calculate_board_score(board_nums: &HashSet<&u32>, draws: &[&u32]) -> u32 {
    let draws_set = draws.iter().copied().collect::<HashSet<_>>();

    let init: u32 = 0;
    let remaining_sum = board_nums
        .difference(&draws_set)
        .fold(init, |total, &value| total + value);

    remaining_sum * (*draws.last().unwrap())
}

fn main() {
    let contents = include_str!("day4.txt");

    let board_size: usize = 5;

    let (draws_raw, boards_raw) = contents.split_once("\n\n").unwrap();

    let draws = draws_raw
        .split(',')
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    let boards = boards_raw
        .split("\n\n")
        .map(|board| {
            board
                .split('\n')
                .map(|row| {
                    row.split_whitespace()
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let board_nums = boards
        .iter()
        .map(|board| board.iter().flatten().collect::<HashSet<_>>())
        .collect::<Vec<_>>();

    let board_combos = boards
        .iter()
        .map(|board| {
            let rows = board
                .iter()
                .map(|row| row.iter().collect::<HashSet<_>>())
                .collect::<Vec<_>>()
                .into_iter();
            let cols = (0..board_size)
                .map(|i| board.iter().map(|row| &row[i]).collect::<HashSet<_>>())
                .collect::<Vec<_>>()
                .into_iter();
            rows.chain(cols).collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    for index in board_size..draws.len() {
        let draw: Vec<&u32> = draws.iter().take(index).collect();
        let draw_set = draw.iter().copied().collect::<HashSet<_>>();

        let bingos = board_combos
            .iter()
            .zip(board_nums.iter())
            .filter(|(board_combo, _)| check_board(&board_combo[..], &draw_set))
            .map(|(_, nums)| nums)
            .collect::<Vec<_>>();

        if bingos.iter().len() > 0 {
            let score = calculate_board_score(bingos.first().unwrap(), &draw);
            println!("Part 1: {:?}", score);
            break;
        }
    }

    for index in (board_size..draws.len()).rev() {
        let draw: Vec<&u32> = draws.iter().take(index).collect();
        let draw_set = draw.iter().copied().collect::<HashSet<_>>();

        let not_bingos = board_combos
            .iter()
            .zip(board_nums.iter())
            .filter(|(board_combo, _)| !check_board(&board_combo[..], &draw_set))
            .map(|(_, nums)| nums)
            .collect::<Vec<_>>();

        if not_bingos.iter().len() > 0 {
            let previous_draw: Vec<&u32> = draws.iter().take(index + 1).collect();
            let score = calculate_board_score(not_bingos.first().unwrap(), &previous_draw);
            println!("Part 2: {:?}", score);
            break;
        }
    }
}
