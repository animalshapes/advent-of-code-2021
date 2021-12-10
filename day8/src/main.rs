use std::collections::{HashMap, HashSet};
use std::hash::Hash;

fn find_by_len<T>(input: &[HashSet<T>], len: usize) -> Option<&HashSet<T>> {
    input.iter().find(|&ele| ele.len() == len)
}

fn union_chain<T>(first: &HashSet<T>, second: &HashSet<T>) -> HashSet<T>
where
    T: Eq + Hash + Copy,
{
    first.union(second).copied().collect()
}

fn diff_chain<T>(first: &HashSet<T>, second: &HashSet<T>) -> HashSet<T>
where
    T: Eq + Hash + Copy,
{
    first.difference(second).copied().collect()
}

fn alphabetize(s: &str) -> String {
    let mut temp = s.chars().collect::<Vec<_>>();
    temp.sort_unstable();
    temp.into_iter().collect::<String>()
}

fn construct_mappings(input: &[&str]) -> HashMap<String, usize> {
    let input_sets: Vec<HashSet<char>> = input
        .iter()
        .map(|&digit| digit.chars().collect::<HashSet<_>>())
        .collect();

    let one = find_by_len(&input_sets, 2).unwrap();
    let seven = find_by_len(&input_sets, 3).unwrap();
    let four = find_by_len(&input_sets, 4).unwrap();
    let eight = find_by_len(&input_sets, 7).unwrap();

    let top = diff_chain(seven, one);
    let four_top = union_chain(four, &top);
    let nine = input_sets
        .iter()
        .find(|&ele| ele.len() == 6 && four_top.is_subset(ele))
        .unwrap();

    let bottom = diff_chain(nine, &four_top);

    let one_top_bot = union_chain(one, &union_chain(&top, &bottom));

    let three = input_sets
        .iter()
        .find(|&ele| ele.len() == 5 && one_top_bot.is_subset(ele))
        .unwrap();

    let mid = diff_chain(three, &one_top_bot);

    let zero = diff_chain(eight, &mid);

    let six = input_sets
        .iter()
        .find(|&ele| ele.len() == 6 && ele != &zero && ele != nine)
        .unwrap();

    let five = input_sets
        .iter()
        .find(|&ele| ele.len() == 5 && ele.is_subset(six))
        .unwrap();

    let two = input_sets
        .iter()
        .find(|&ele| ele.len() == 5 && ele != three && ele != five)
        .unwrap();

    let values = vec![
        alphabetize(&zero.iter().collect::<String>()),
        alphabetize(&one.iter().collect::<String>()),
        alphabetize(&two.iter().collect::<String>()),
        alphabetize(&three.iter().collect::<String>()),
        alphabetize(&four.iter().collect::<String>()),
        alphabetize(&five.iter().collect::<String>()),
        alphabetize(&six.iter().collect::<String>()),
        alphabetize(&seven.iter().collect::<String>()),
        alphabetize(&eight.iter().collect::<String>()),
        alphabetize(&nine.iter().collect::<String>()),
    ];
    let mut mappings: HashMap<String, usize> = HashMap::new();

    for (i, value) in values.iter().enumerate() {
        mappings.insert(value.clone(), i);
    }
    mappings
}

fn main() {
    let contents = include_str!("day8.txt");

    let rows: Vec<&str> = contents.split('\n').collect();

    let parsed_rows: Vec<Vec<Vec<&str>>> = rows
        .iter()
        .map(|&row| {
            row.split(" | ")
                .map(|side| side.split_whitespace().collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .collect();

    let wires: Vec<&Vec<&str>> = parsed_rows.iter().map(|row| row.first().unwrap()).collect();
    let digits_vec: Vec<&Vec<&str>> = parsed_rows.iter().map(|row| row.last().unwrap()).collect();

    let part_1 = digits_vec
        .iter()
        .flat_map(|&set| set.iter().map(|&ele| ele.len()))
        .filter(|&ele| ele == 2 || ele == 3 || ele == 4 || ele == 7)
        .count();

    println!("Part 1: {:?}", part_1);

    let part_2: usize = wires
        .iter()
        .zip(digits_vec.iter())
        .map(|(&wire, &digits)| {
            let mappings = construct_mappings(wire);
            let value = digits
                .iter()
                .map(|&digit| mappings.get(&alphabetize(digit)).unwrap())
                .fold(0, |acc, ele| acc * 10 + *ele);
            value
        })
        .sum();

    println!("Part 2: {:?}", part_2);
}
