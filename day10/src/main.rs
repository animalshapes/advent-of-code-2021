use std::collections::{HashSet, VecDeque};

use itertools::{Either, Itertools};

#[derive(Debug)]
enum ParseResult {
    CompletionScore(i64),
    SyntaxErrorScore(i64),
}

fn calculate_scores(phrase: &[char]) -> ParseResult {
    let open_symbols: HashSet<char> = HashSet::from(['(', '[', '{', '<']);

    let mut stack: VecDeque<char> = VecDeque::new();

    for symbol in phrase {
        if open_symbols.contains(symbol) {
            stack.push_back(*symbol);
        } else {
            match (symbol, stack.pop_back()) {
                (')', Some(x)) => {
                    if x != '(' {
                        return ParseResult::SyntaxErrorScore(3);
                    }
                }
                (']', Some(x)) => {
                    if x != '[' {
                        return ParseResult::SyntaxErrorScore(57);
                    }
                }
                ('}', Some(x)) => {
                    if x != '{' {
                        return ParseResult::SyntaxErrorScore(1197);
                    }
                }
                ('>', Some(x)) => {
                    if x != '<' {
                        return ParseResult::SyntaxErrorScore(25137);
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

    ParseResult::CompletionScore(completion_score)
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
        .partition_map(|val| match val {
            ParseResult::SyntaxErrorScore(val) => Either::Left(val),
            ParseResult::CompletionScore(val) => Either::Right(val),
        });

    let syntax_score: i64 = syntax_error_scores.iter().sum();

    println!("Part 1: {:?}", syntax_score);

    completion_scores.sort_unstable();
    let completion_score = completion_scores[completion_scores.len() / 2];

    println!("Part 2: {:?}", completion_score);
}
