use std::cmp;
use std::collections::HashSet;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point {
    row: u32,
    col: u32,
}

impl Point {
    fn fold(&mut self, fold: &Fold) {
        match fold.direction {
            Direction::Up => {
                if self.row > fold.location {
                    self.row = 2 * fold.location - self.row;
                }
            }
            Direction::Left => {
                if self.col > fold.location {
                    self.col = 2 * fold.location - self.col;
                }
            }
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Left,
}

#[derive(Debug)]
struct Fold {
    direction: Direction,
    location: u32,
}

fn main() {
    let contents = include_str!("day13.txt").trim_end();

    let (folds_raw, points_raw): (Vec<&str>, Vec<&str>) = contents
        .split('\n')
        .partition(|&ele| ele.contains("fold along"));

    let mut points: Vec<Point> = points_raw
        .into_iter()
        .filter(|&ele| !ele.is_empty())
        .map(|point| {
            let point_tuple = point.split_once(',').expect("unexpected point format");
            let col = point_tuple
                .0
                .parse::<u32>()
                .expect("unexpected number format");
            let row = point_tuple
                .1
                .parse::<u32>()
                .expect("unexpected number format");
            Point { row, col }
        })
        .collect();

    let folds: Vec<Fold> = folds_raw
        .into_iter()
        .map(|ele| {
            let values = ele
                .split_whitespace()
                .last()
                .expect("unexpected fold format");
            let fold_tuple = values.split_once('=').expect("unexpected fold axis format");
            let direction = match fold_tuple.0 {
                "x" => Direction::Left,
                "y" => Direction::Up,
                _ => panic!("unexpected fold axis"),
            };
            let location = fold_tuple
                .1
                .parse::<u32>()
                .expect("unexpected fold location");
            Fold {
                direction,
                location,
            }
        })
        .collect();

    for (i, fold) in folds.iter().enumerate() {
        for point in points.iter_mut() {
            point.fold(fold);
        }

        if i == 0 {
            let non_dupes: HashSet<&Point> = points.iter().collect();
            println!("Part 1: {:#?}", non_dupes.len());
        }
    }

    let (max_rows, max_cols) = points.iter().fold((0, 0), |acc, point| {
        (cmp::max(acc.0, point.row), cmp::max(acc.1, point.col))
    });

    let mut visual = vec![vec![' '; (max_cols + 1) as usize]; (max_rows + 1) as usize];

    for point in &points {
        visual[point.row as usize][point.col as usize] = '#';
    }

    let output: Vec<String> = visual
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect();

    println!("Part 2: ");
    for line in output {
        println!("{}", line);
    }
}
