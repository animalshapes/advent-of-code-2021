use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coordinate {
    row: usize,
    column: usize,
}

fn get_valid_neighbors(grid: &[Vec<u32>], coord: &Coordinate) -> Vec<Coordinate> {
    let rows = grid.len();
    let columns = grid[0].len();

    let mut vec: Vec<Coordinate> = Vec::new();

    let not_first_row = coord.row > 0;
    let not_last_row = coord.row < rows - 1;
    let not_first_col = coord.column > 0;
    let not_last_col = coord.column < columns - 1;

    if not_first_row {
        vec.push(Coordinate {
            row: coord.row - 1,
            column: coord.column,
        });
    }
    if not_last_row {
        vec.push(Coordinate {
            row: coord.row + 1,
            column: coord.column,
        });
    }
    if not_first_col {
        vec.push(Coordinate {
            row: coord.row,
            column: coord.column - 1,
        });
    }
    if not_last_col {
        vec.push(Coordinate {
            row: coord.row,
            column: coord.column + 1,
        });
    }
    if not_first_row && not_first_col {
        vec.push(Coordinate {
            row: coord.row - 1,
            column: coord.column - 1,
        });
    }
    if not_first_row && not_last_col {
        vec.push(Coordinate {
            row: coord.row - 1,
            column: coord.column + 1,
        });
    }
    if not_last_row && not_first_col {
        vec.push(Coordinate {
            row: coord.row + 1,
            column: coord.column - 1,
        });
    }
    if not_last_row && not_last_col {
        vec.push(Coordinate {
            row: coord.row + 1,
            column: coord.column + 1,
        });
    }

    vec
}

fn increment_coordinate(grid: &mut [Vec<u32>], coord: &Coordinate) {
    grid[coord.row][coord.column] += 1;
}

fn reset_coordinate(grid: &mut [Vec<u32>], coord: &Coordinate) {
    grid[coord.row][coord.column] = 0;
}

fn get_grid_value(grid: &[Vec<u32>], coord: &Coordinate) -> u32 {
    grid[coord.row][coord.column]
}

fn run_step(grid: &mut [Vec<u32>]) -> usize {
    let mut flashed: HashSet<Coordinate> = HashSet::new();
    let mut deq: VecDeque<Coordinate> = VecDeque::new();

    for (row_ind, row) in grid.iter_mut().enumerate() {
        for (col_ind, val) in row.iter_mut().enumerate() {
            *val += 1;
            if *val > 9 {
                let current = Coordinate {
                    row: row_ind,
                    column: col_ind,
                };
                deq.push_back(current);
                flashed.insert(current);
            }
        }
    }

    while let Some(coord) = deq.pop_front() {
        let neighbors = get_valid_neighbors(grid, &coord);
        for neighbor in neighbors {
            increment_coordinate(grid, &neighbor);
            if !flashed.contains(&neighbor) && get_grid_value(grid, &neighbor) > 9 {
                flashed.insert(neighbor);
                deq.push_back(neighbor);
            }
        }
    }

    for coord in &flashed {
        reset_coordinate(grid, coord);
    }

    flashed.len()
}

fn main() {
    let contents = include_str!("day11.txt");

    let mut grid = contents
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|ele| ele.to_digit(10).expect("not a digit"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut total_flashes: usize = 0;
    let mut counter = 0;
    loop {
        counter += 1;
        let flashes = run_step(&mut grid[..][..]);
        total_flashes += flashes;

        if counter == 100 {
            println!("Part 1: {:?}", total_flashes);
        }

        if flashes == 100 {
            println!("Part 2: {:?}", counter);
            break;
        }
    }
}
