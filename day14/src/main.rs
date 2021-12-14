use itertools::Itertools;
use std::collections::HashMap;

fn char_windows(src: &str, win_size: usize) -> impl Iterator<Item = &str> {
    src.char_indices().flat_map(move |(from, _)| {
        src[from..]
            .char_indices()
            .skip(win_size - 1)
            .next()
            .map(|(to, c)| &src[from..from + to + c.len_utf8()])
    })
}

fn template_step(template: &str, map: &HashMap<&str, &str>) -> String {
    let windows_1 = char_windows(template, 1);
    let windows_2 = char_windows(template, 2);
    let insertions = windows_2.map(|window| *map.get(window).expect("unexpected pattern"));
    windows_1.interleave(insertions).collect()
}

fn get_counts(template: &str) -> [i32; 10] {
    let counts = template.chars().fold([0; 10], |mut acc, ele| {
        match ele {
            'C' => acc[0] += 1,
            'H' => acc[1] += 1,
            'K' => acc[2] += 1,
            'B' => acc[3] += 1,
            'S' => acc[4] += 1,
            'F' => acc[5] += 1,
            'O' => acc[6] += 1,
            'V' => acc[7] += 1,
            'N' => acc[8] += 1,
            'P' => acc[9] += 1,
            _ => panic!("unexpected input"),
        }
        acc
    });
    counts
}

fn main() {
    let contents = include_str!("day14.txt").trim_end();

    let (template, rules) = contents.split_once("\n\n").expect("unexpected format");

    let mut map: HashMap<&str, &str> = HashMap::new();

    rules.split('\n').for_each(|rule| {
        let (key, value) = rule.split_once(" -> ").expect("unexpected rule format");
        map.insert(key, value);
    });

    let mut current_template = template.to_owned();
    for _ in 0..10 {
        current_template = template_step(&current_template, &map);
    }

    let counts = get_counts(&current_template);

    let mut min = i32::MAX;
    let mut max = 0;
    for ele in counts {
        if ele < min {
            min = ele;
        } else if ele > max {
            max = ele;
        }
    }

    println!("{:?}", max - min);
}
