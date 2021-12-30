fn get_potential_neighbors(row: usize, col: usize) -> Vec<Option<(usize, usize)>> {
    [
        (row.checked_sub(1), col.checked_sub(1)),
        (row.checked_sub(1), Some(col)),
        (row.checked_sub(1), Some(col + 1)),
        (Some(row), col.checked_sub(1)),
        (Some(row), Some(col)),
        (Some(row), Some(col + 1)),
        (Some(row + 1), col.checked_sub(1)),
        (Some(row + 1), Some(col)),
        (Some(row + 1), Some(col + 1)),
    ]
    .iter()
    .map(|(row, col)| match (row, col) {
        (Some(row), Some(col)) => Some((*row, *col)),
        (_, _) => None,
    })
    .collect()
}

fn run_processing(image: &[Vec<char>], key: &str, default: char) -> Vec<Vec<char>> {
    let mut new_image = vec![vec!['d'; image[0].len()]; image.len()];

    for (row_index, row) in new_image.iter_mut().enumerate() {
        for (col_index, element) in row.iter_mut().enumerate() {
            let index_str: String = get_potential_neighbors(row_index, col_index)
                .iter()
                .map(|neighbor| match neighbor {
                    None => default,
                    Some((new_row, new_col)) => {
                        match image.get(*new_row).and_then(|ele| ele.get(*new_col)) {
                            None => default,
                            Some(&ele) => ele,
                        }
                    }
                })
                .collect();
            let index = usize::from_str_radix(&index_str, 2).expect("not a number");
            *element = key.as_bytes()[index] as char;
        }
    }

    new_image
}

fn count_total(image: &[Vec<char>]) -> usize {
    image
        .iter()
        .map(|row| row.iter().filter(|&ele| *ele == '1').count())
        .sum()
}

fn main() {
    let contents = include_str!("day20.txt").trim_end();

    let mut split = contents.split("\n\n");

    let key: String = split
        .next()
        .unwrap()
        .chars()
        .map(|ele| if ele == '#' { '1' } else { '0' })
        .collect();
    let mut image: Vec<Vec<char>> = split
        .next()
        .unwrap()
        .split('\n')
        .map(|row| {
            row.chars()
                .map(|ele| if ele == '#' { '1' } else { '0' })
                .collect()
        })
        .collect();

    let num_rounds: usize = 50;

    for round in 0..num_rounds {
        let default = if round % 2 == 0 {
            *key.as_bytes().last().unwrap() as char
        } else {
            *key.as_bytes().first().unwrap() as char
        };
        image.insert(0, vec![default; image[0].len()]);
        image.push(vec![default; image[0].len()]);
        image.iter_mut().for_each(|row| {
            row.insert(0, default);
            row.push(default);
        });
        image = run_processing(&image, &key, default);

        if round == 1 {
            let p1_total = count_total(&image);
            println!("Part 1: {:?}", p1_total);
        }
    }

    let p2_total = count_total(&image);
    println!("Part 2: {:?}", p2_total);
}
