use std::collections::{HashSet, VecDeque};

use itertools::{Either, Itertools};

fn calculate_scores(phrase: &[char]) -> Either<i64, i64> {
    let open_symbols: HashSet<char> = HashSet::from(['(', '[', '{', '<']);

    let mut stack: VecDeque<char> = VecDeque::new();

    for symbol in phrase {
        if open_symbols.contains(symbol) {
            stack.push_back(*symbol);
        } else {
            match (symbol, stack.pop_back()) {
                (')', Some(x)) => {
                    if x != '(' {
                        return Either::Left(3);
                    }
                }
                (']', Some(x)) => {
                    if x != '[' {
                        return Either::Left(57);
                    }
                }
                ('}', Some(x)) => {
                    if x != '{' {
                        return Either::Left(1197);
                    }
                }
                ('>', Some(x)) => {
                    if x != '<' {
                        return Either::Left(25137);
                    }
                }
                _ => (),
            }
        }
    }

    let completion_score = stack
        .iter()
        .rev()
        .map(|ele| match ele {
            '(' => 1,
            '[' => 2,
            '{' => 3,
            '<' => 4,
            _ => panic!("unexpected"),
        })
        .fold(0, |acc, value| acc * 5 + value);

    Either::Right(completion_score)
}

fn main() {
    let contents = include_str!("day10.txt");

    let phrases: Vec<Vec<char>> = contents
        .split('\n')
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect();

    let (syntax_error_scores, mut completion_scores): (Vec<i64>, Vec<i64>) = phrases
        .iter()
        .map(|phrase| calculate_scores(phrase))
        .partition_map(|val| val);

    let syntax_score: i64 = syntax_error_scores.iter().sum();

    println!("Part 1: {:?}", syntax_score);

    completion_scores.sort_unstable();
    let completion_score = completion_scores[completion_scores.len() / 2];

    println!("Part 2: {:?}", completion_score);
}
