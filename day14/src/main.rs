use itertools::Itertools;
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
    c: u64,
    h: u64,
    k: u64,
    b: u64,
    s: u64,
    f: u64,
    o: u64,
    v: u64,
    n: u64,
    p: u64,
}

impl LetterCounts {
    fn new() -> LetterCounts {
        LetterCounts {
            c: 0,
            h: 0,
            k: 0,
            b: 0,
            s: 0,
            f: 0,
            o: 0,
            v: 0,
            n: 0,
            p: 0,
        }
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
            "C" => self.c += 1,
            "H" => self.h += 1,
            "K" => self.k += 1,
            "B" => self.b += 1,
            "S" => self.s += 1,
            "F" => self.f += 1,
            "O" => self.o += 1,
            "V" => self.v += 1,
            "N" => self.n += 1,
            "P" => self.p += 1,
            _ => panic!("unexpected input"),
        }
    }

    fn add(&mut self, other: &LetterCounts) {
        self.c += other.c;
        self.h += other.h;
        self.k += other.k;
        self.b += other.b;
        self.s += other.s;
        self.f += other.f;
        self.o += other.o;
        self.v += other.v;
        self.n += other.n;
        self.p += other.p;
    }
}

fn process_template(count_map: &HashMap<&str, LetterCounts>, template: &str) -> LetterCounts {
    let mut counts = LetterCounts::from_str(template);
    for key in char_windows(template, 2) {
        counts.add(count_map.get(key).expect("must exist"));
    }
    counts
}

fn main() {
    let contents = include_str!("day14_test.txt").trim_end();

    let (template, rules) = contents.split_once("\n\n").expect("unexpected format");

    let mut char_map: HashMap<&str, &str> = HashMap::new();
    let mut child_map: HashMap<&str, [String; 2]> = HashMap::new();

    rules.split('\n').for_each(|rule| {
        let (key, value) = rule.split_once(" -> ").expect("unexpected rule format");
        char_map.insert(key, value);
        let mut inserted = key.to_owned();
        inserted.insert_str(1, value);
        let first: String = inserted
            .clone()
            .char_indices()
            .map(|(_, ele)| ele)
            .take(2)
            .collect();
        let second: String = inserted
            .clone()
            .char_indices()
            .map(|(_, ele)| ele)
            .skip(1)
            .take(2)
            .collect();
        child_map.insert(key, [first, second]);
    });

    let mut counter: HashMap<&str, LetterCounts> = HashMap::new();

    for &key in char_map.keys() {
        let added_char = *char_map.get(key).expect("impossible");
        counter
            .entry(key)
            .or_insert_with(LetterCounts::new)
            .increment(added_char);
    }

    for _ in 0..39 {
        let mut next_counter: HashMap<&str, LetterCounts> = counter.clone();
        for &key in char_map.keys() {
            let children = child_map.get(key).expect("impossible");
            for child in children {
                let child_slice = &child[..];
                let to_add = counter.get(child_slice).expect("child must exist");
                next_counter
                    .entry(key)
                    .and_modify(|counts| counts.add(to_add));
            }
        }
        counter = next_counter;
    }

    let result = process_template(&counter, template);

    println!("{:?}", counter);
}
