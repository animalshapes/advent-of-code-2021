fn move_east(board: &mut [Vec<char>]) -> bool {
    let mut flag = false;
    let row_len = board[0].len();
    for row in board {
        let mut to_swap: Vec<(usize, usize)> = Vec::new();
        for index in (0..row.len()).rev() {
            if row[index] != '>' {
                continue;
            }
            let next_index = (index + 1) % row_len;
            if row[next_index] == '.' {
                to_swap.push((index, next_index));
                flag = true;
            }
        }
        while let Some((index, next_index)) = to_swap.pop() {
            row.swap(index, next_index);
        }
    }
    flag
}

fn move_south(board: &mut [Vec<char>]) -> bool {
    let mut flag = false;
    let num_rows = board.len();
    for col_index in 0..board[0].len() {
        let column = board
            .iter_mut()
            .map(|row| row[col_index])
            .collect::<Vec<_>>();

        let mut to_swap: Vec<(usize, usize)> = Vec::new();

        for index in (0..num_rows).rev() {
            if column[index] != 'v' {
                continue;
            }
            let next_index = (index + 1) % num_rows;
            if column[next_index] == '.' {
                to_swap.push((index, next_index));
                flag = true;
            }
        }

        while let Some((index, next_index)) = to_swap.pop() {
            board[index][col_index] = '.';
            board[next_index][col_index] = 'v';
        }
    }
    flag
}

fn run_step(board: &mut [Vec<char>]) -> bool {
    let east = move_east(board);
    let south = move_south(board);
    east || south
}

// fn print_board(board: &[Vec<char>]) {
//     for row in board {
//         println!("{:?}", row);
//     }
// }

fn main() {
    let contents = include_str!("day25.txt");

    let mut board: Vec<Vec<char>> = contents
        .lines()
        .map(|line| line.chars().collect())
        .collect();

    let mut counter: usize = 1;
    while run_step(&mut board) {
        counter += 1;
    }
    println!("Part 1: {:?}", counter);
}
