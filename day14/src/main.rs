use std::collections::HashMap;

type LetterCounts = [u64; 10];

fn char_windows(src: &str, win_size: usize) -> impl Iterator<Item = &str> {
    src.char_indices().flat_map(move |(from, _)| {
        src[from..]
            .char_indices()
            .nth(win_size - 1)
            .map(|(to, c)| &src[from..from + to + c.len_utf8()])
    })
}

fn increment_counts(input: &mut LetterCounts, value: &str) {
    match value {
        "B" => input[0] += 1,
        "C" => input[1] += 1,
        "F" => input[2] += 1,
        "H" => input[3] += 1,
        "K" => input[4] += 1,
        "N" => input[5] += 1,
        "O" => input[6] += 1,
        "P" => input[7] += 1,
        "S" => input[8] += 1,
        "V" => input[9] += 1,
        _ => panic!("unexpected input"),
    }
}

fn add_counts(input: &mut LetterCounts, other: &LetterCounts) {
    for (index, val) in input.iter_mut().enumerate() {
        *val += other[index];
    }
}

fn initialize_counts_from_str(value: &str) -> LetterCounts {
    let mut start: LetterCounts = [0; 10];
    for ele in value.chars() {
        increment_counts(&mut start, &ele.to_string())
    }

    start
}

fn process_template(count_map: &HashMap<&str, LetterCounts>, template: &str) -> u64 {
    let mut counts = initialize_counts_from_str(template);
    for key in char_windows(template, 2) {
        add_counts(&mut counts, count_map.get(key).expect("must exist"));
    }

    let max = counts.iter().max().unwrap();
    let min = counts.iter().min().unwrap();
    max - min
}

fn polymer_insertion(template: &str, num_steps: usize, char_map: &HashMap<&str, &str>) -> u64 {
    let child_map: HashMap<&str, [String; 2]> = char_map
        .keys()
        .zip(char_map.values())
        .map(|(&key, &value)| {
            let mut inserted = key.to_owned();
            inserted.insert_str(1, value);
            let base = inserted.chars();
            let first: String = base.clone().take(2).collect();
            let second: String = base.clone().skip(1).take(2).collect();
            (key, [first, second])
        })
        .collect();

    let mut letter_counter: HashMap<&str, LetterCounts> =
        char_map.keys().map(|&key| (key, [0; 10])).collect();

    for _ in 0..num_steps {
        let mut next_counter: HashMap<&str, LetterCounts> = HashMap::new();
        for &key in char_map.keys() {
            next_counter.entry(key).or_insert_with(|| [0; 10]);

            let children = child_map.get(key).expect("impossible");
            for child in children {
                let child_slice = &child[..];
                let to_add = letter_counter.get(child_slice).expect("child must exist");
                next_counter
                    .entry(key)
                    .and_modify(|counts| add_counts(counts, to_add));
            }
            let added_char = *char_map.get(key).expect("impossible");
            next_counter
                .entry(key)
                .and_modify(|count| increment_counts(count, added_char));
        }
        letter_counter = next_counter;
    }

    process_template(&letter_counter, template)
}

fn main() {
    let contents = include_str!("day14.txt").trim_end();

    let (template, rules) = contents.split_once("\n\n").expect("unexpected file format");

    let char_map: HashMap<&str, &str> = rules
        .lines()
        .map(|line| line.split_once(" -> ").expect("unexpected rule format"))
        .collect();

    let p1_result = polymer_insertion(template, 10, &char_map);
    println!("Part 1: {:?}", p1_result);

    let p2_result = polymer_insertion(template, 40, &char_map);
    println!("Part 2: {:?}", p2_result);
}
