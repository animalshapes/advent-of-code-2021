use std::collections::{HashSet, VecDeque};

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

    let (syntax_error_scores, completion_scores): (Vec<ParseResult>, Vec<ParseResult>) = phrases
        .iter()
        .map(|phrase| calculate_scores(phrase))
        .partition(|val| match val {
            ParseResult::SyntaxErrorScore(_) => true,
            ParseResult::CompletionScore(_) => false,
        });

    let syntax_score: i64 = syntax_error_scores
        .iter()
        .map(|ele| {
            if let ParseResult::SyntaxErrorScore(val) = ele {
                *val
            } else {
                panic!("partition failed!")
            }
        })
        .sum();

    println!("Part 1: {:?}", syntax_score);

    let mut unwrapped_completion_scores: Vec<i64> = completion_scores
        .iter()
        .map(|ele| {
            if let ParseResult::CompletionScore(val) = ele {
                *val
            } else {
                panic!("partition failed!")
            }
        })
        .collect();

    unwrapped_completion_scores.sort_unstable();
    let completion_score = unwrapped_completion_scores[unwrapped_completion_scores.len() / 2];

    println!("Part 2: {:?}", completion_score);
}
