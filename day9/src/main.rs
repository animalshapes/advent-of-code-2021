use std::collections::{HashSet, VecDeque};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Coordinate {
    row: usize,
    column: usize,
}

fn get_grid_value(grid: &[Vec<u32>], coord: &Coordinate) -> u32 {
    grid[coord.row][coord.column]
}

fn get_valid_neighbors(grid: &[Vec<u32>], coord: &Coordinate) -> Vec<Coordinate> {
    let rows = grid.len();
    let columns = grid[0].len();

    let mut vec: Vec<Coordinate> = Vec::new();

    if coord.row > 0 {
        vec.push(Coordinate {
            row: coord.row - 1,
            column: coord.column,
        });
    }
    if coord.row < rows - 1 {
        vec.push(Coordinate {
            row: coord.row + 1,
            column: coord.column,
        });
    }
    if coord.column > 0 {
        vec.push(Coordinate {
            row: coord.row,
            column: coord.column - 1,
        });
    }
    if coord.column < columns - 1 {
        vec.push(Coordinate {
            row: coord.row,
            column: coord.column + 1,
        });
    }

    vec
}

fn get_basin_size(grid: &[Vec<u32>], start: Coordinate) -> usize {
    let mut visited = HashSet::from([start]);
    let mut deq = VecDeque::from([start]);

    let mut size: usize = 1;
    while let Some(coord) = deq.pop_front() {
        let current_val = get_grid_value(grid, &coord);
        let neighbors = get_valid_neighbors(grid, &coord);

        for neighbor in neighbors {
            let neighbor_val = get_grid_value(grid, &neighbor);
            if !visited.contains(&neighbor) && neighbor_val != 9 && neighbor_val > current_val {
                deq.push_back(neighbor);
                visited.insert(neighbor);
                size += 1;
            }
        }
    }

    size
}

fn main() {
    let contents = include_str!("day9.txt");

    let grid = contents
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|ele| ele.to_digit(10).expect("not a digit"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let rows = grid.len();
    let columns = grid[0].len();

    let mut basin_sizes: Vec<usize> = Vec::new();
    let mut risk: u32 = 0;

    for row in 0..rows {
        for column in 0..columns {
            let coord = Coordinate { row, column };
            let valid_neighbors = get_valid_neighbors(&grid, &coord);

            let current_val = get_grid_value(&grid, &coord);

            if !valid_neighbors
                .iter()
                .any(|neighbor| get_grid_value(&grid, neighbor) <= current_val)
            {
                risk += current_val + 1;
                basin_sizes.push(get_basin_size(&grid, coord));
            }
        }
    }

    println!("Part 1: {:?}", risk);

    basin_sizes.sort_unstable();
    let basin_prod: usize = basin_sizes.iter().rev().take(3).product();

    println!("Part 2: {:?}", basin_prod);
}
