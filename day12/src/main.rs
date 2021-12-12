use std::collections::{HashMap, VecDeque};

fn is_large_cave(cave: &str) -> bool {
    cave.chars().all(|c| c.is_ascii_uppercase())
}

fn convert_edges_to_map(edges: Vec<&str>) -> HashMap<&str, Vec<&str>> {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for edge in edges {
        let mut sides = edge.split('-');
        let first = sides.next().expect("no first node");
        let second = sides.next().expect("no second node");

        map.entry(first).or_insert_with(Vec::new).push(second);
        map.entry(second).or_insert_with(Vec::new).push(first);
    }
    map
}

fn traverse_map_p1<'a, 'b>(map: &'b HashMap<&'a str, Vec<&'a str>>) -> i32 {
    let mut deq: VecDeque<(&str, Vec<&str>)> = VecDeque::from([("start", vec!["start"])]);
    let mut paths: i32 = 0;

    while let Some((id, path)) = deq.pop_front() {
        let neighbors = map.get(id).expect("node does not exist in map");
        for &neighbor in neighbors {
            if is_large_cave(neighbor) || !path.contains(&neighbor) {
                if neighbor == "end" {
                    paths += 1
                } else {
                    let mut new_path = path.clone();
                    new_path.push(neighbor);
                    deq.push_back((neighbor, new_path))
                }
            }
        }
    }

    paths
}

fn traverse_map_p2<'a, 'b>(map: &'b HashMap<&'a str, Vec<&'a str>>) -> i32 {
    let mut deq: VecDeque<(&str, Vec<&str>, bool)> =
        VecDeque::from([("start", vec!["start"], false)]);
    let mut paths: i32 = 0;

    while let Some((id, path, dupe)) = deq.pop_front() {
        let neighbors = map.get(id).expect("node does not exist in map");
        for &neighbor in neighbors {
            if dupe {
                if is_large_cave(neighbor) || !path.contains(&neighbor) {
                    if neighbor == "end" {
                        paths += 1;
                    } else {
                        let mut new_path = path.clone();
                        new_path.push(neighbor);
                        deq.push_back((neighbor, new_path, dupe))
                    }
                }
            } else if neighbor != "start" {
                if neighbor == "end" {
                    paths += 1;
                } else {
                    let mut new_path = path.clone();
                    new_path.push(neighbor);
                    let dupe = path
                        .iter()
                        .copied()
                        .filter(|&cave| !is_large_cave(cave))
                        .any(|cave| cave == neighbor);
                    deq.push_back((neighbor, new_path, dupe));
                }
            }
        }
    }

    paths
}

fn main() {
    let contents = include_str!("day12.txt");
    let edges: Vec<&str> = contents.split('\n').collect();

    let map = convert_edges_to_map(edges);

    let paths_p1 = traverse_map_p1(&map);
    println!("Part 1: {:?}", paths_p1);

    let paths_p2 = traverse_map_p2(&map);
    println!("Part 2: {:?}", paths_p2);
}
