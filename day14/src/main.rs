use std::collections::HashMap;

fn char_windows(src: &str, win_size: usize) -> impl Iterator<Item = &str> {
    src.char_indices().flat_map(move |(from, _)| {
        src[from..]
            .char_indices()
            .nth(win_size - 1)
            .map(|(to, c)| &src[from..from + to + c.len_utf8()])
    })
}

#[derive(Clone, Copy, Debug)]
struct LetterCounts {
    values: [u64; 10],
}

impl LetterCounts {
    fn new() -> LetterCounts {
        LetterCounts { values: [0; 10] }
    }

    fn from_str(value: &str) -> LetterCounts {
        let mut start = LetterCounts::new();
        value
            .char_indices()
            .for_each(|(_, ele)| start.increment(&ele.to_string()));
        start
    }

    fn increment(&mut self, value: &str) {
        match value {
            "B" => self.values[0] += 1,
            "C" => self.values[1] += 1,
            "F" => self.values[2] += 1,
            "H" => self.values[3] += 1,
            "K" => self.values[4] += 1,
            "N" => self.values[5] += 1,
            "O" => self.values[6] += 1,
            "P" => self.values[7] += 1,
            "S" => self.values[8] += 1,
            "V" => self.values[9] += 1,
            _ => panic!("unexpected input"),
        }
    }

    fn add(&mut self, other: &LetterCounts) {
        for (index, val) in self.values.iter_mut().enumerate() {
            *val += other.values[index];
        }
    }
}

fn process_template(count_map: &HashMap<&str, LetterCounts>, template: &str) -> u64 {
    let mut counts = LetterCounts::from_str(template);
    for key in char_windows(template, 2) {
        counts.add(count_map.get(key).expect("must exist"));
    }

    let max = counts.values.iter().max().unwrap();
    let min = counts.values.iter().min().unwrap();
    max - min
}

fn main() {
    let contents = include_str!("day14.txt").trim_end();

    let (template, rules) = contents.split_once("\n\n").expect("unexpected format");

    let mut char_map: HashMap<&str, &str> = HashMap::new();
    let mut child_map: HashMap<&str, [String; 2]> = HashMap::new();

    for rule in rules.lines() {
        let (key, value) = rule.split_once(" -> ").expect("unexpected rule format");
        char_map.insert(key, value);

        let mut inserted = key.to_owned();
        inserted.insert_str(1, value);

        let base = inserted.char_indices().map(|(_, ele)| ele);
        let first: String = base.clone().take(2).collect();
        let second: String = base.clone().skip(1).take(2).collect();
        child_map.insert(key, [first, second]);
    }

    let mut counter: HashMap<&str, LetterCounts> = HashMap::new();
    for &key in char_map.keys() {
        counter.entry(key).or_insert_with(LetterCounts::new);
    }

    for i in 0..40 {
        let mut next_counter: HashMap<&str, LetterCounts> = HashMap::new();
        for &key in char_map.keys() {
            next_counter.entry(key).or_insert_with(LetterCounts::new);

            let children = child_map.get(key).expect("impossible");
            for child in children {
                let child_slice = &child[..];
                let to_add = counter.get(child_slice).expect("child must exist");
                next_counter
                    .entry(key)
                    .and_modify(|counts| counts.add(to_add));
            }
            let added_char = *char_map.get(key).expect("impossible");
            next_counter
                .entry(key)
                .and_modify(|count| count.increment(added_char));
        }
        counter = next_counter;

        if i == 9 {
            let p1_result = process_template(&counter, template);
            println!("Part 1: {:?}", p1_result);
        }
    }

    let p2_result = process_template(&counter, template);
    println!("Part 2: {:?}", p2_result);
}
