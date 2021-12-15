use std::cmp::Reverse;
use std::collections::BinaryHeap;

fn get_valid_neighbors(point: (usize, usize), dimension: usize) -> Vec<(usize, usize)> {
    let mut neighbors: Vec<(usize, usize)> = Vec::new();
    if point.0 > 0 {
        neighbors.push((point.0 - 1, point.1));
    }
    if point.0 < dimension - 1 {
        neighbors.push((point.0 + 1, point.1));
    }
    if point.1 > 0 {
        neighbors.push((point.0, point.1 - 1));
    }
    if point.1 < dimension - 1 {
        neighbors.push((point.0, point.1 + 1));
    }

    neighbors
}

fn shortest_path(grid: &[Vec<u32>]) -> u32 {
    let mut heap: BinaryHeap<Reverse<(u32, (usize, usize))>> =
        BinaryHeap::from([Reverse((0, (0, 0)))]);

    let rows = grid.len();
    let cols = grid[0].len();

    let mut distances = vec![vec![u32::MAX; cols]; rows];
    distances[0][0] = 0;

    while let Some(Reverse((distance, point))) = heap.pop() {
        if distance > distances[point.0][point.1] {
            continue;
        }
        let neighbors = get_valid_neighbors(point, rows);
        for neighbor in neighbors {
            let neighbor_distance = &mut distances[neighbor.0][neighbor.1];
            let neighbor_risk = grid[neighbor.0][neighbor.1];
            if distance + neighbor_risk < *neighbor_distance {
                *neighbor_distance = distance + neighbor_risk;
                heap.push(Reverse((*neighbor_distance, neighbor)));
            }
        }
    }

    distances[rows - 1][cols - 1]
}

fn main() {
    let contents = include_str!("day15.txt").trim_end();

    const DIMENSION: usize = 100;

    let mut grid = vec![vec![0; DIMENSION]; DIMENSION];

    contents
        .split('\n')
        .enumerate()
        .for_each(|(row_index, row)| {
            row.char_indices().for_each(|(col_index, ele)| {
                grid[row_index][col_index] = ele.to_digit(10).expect("not a number");
            })
        });

    println!("Part 1: {:#?}", shortest_path(&grid));

    let mut larger_grid = vec![vec![0; DIMENSION * 5]; DIMENSION * 5];

    for row in 0..DIMENSION * 5 {
        let origin_val_row = row % DIMENSION;
        let num_row_replications = (row / DIMENSION) as u32;
        for col in 0..DIMENSION * 5 {
            let origin_val_col = col % DIMENSION;
            let num_col_replications = (col / DIMENSION) as u32;
            if num_row_replications == 0 && num_col_replications == 0 {
                larger_grid[row][col] = grid[row][col];
            } else {
                let new_val = (grid[origin_val_row][origin_val_col]
                    + num_row_replications
                    + num_col_replications)
                    % 9;
                larger_grid[row][col] = if new_val != 0 { new_val } else { 9 };
            }
        }
    }

    println!("Part 2: {:#?}", shortest_path(&larger_grid));
}
