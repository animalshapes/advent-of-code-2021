use std::collections::{HashMap, HashSet, VecDeque};

fn is_large_cave(cave: &str) -> bool {
    cave.chars().all(|c| c.is_ascii_uppercase())
}

fn convert_edges_to_map(edges: Vec<&str>) -> HashMap<&str, Vec<&str>> {
    let mut map: HashMap<&str, Vec<&str>> = HashMap::new();
    for edge in edges {
        let mut sides = edge.split('-');
        let first = sides.next().expect("no first node");
        let second = sides.next().expect("no second node");

        map.entry(first).or_insert(Vec::new()).push(second);
        map.entry(second).or_insert(Vec::new()).push(first);
    }
    map
}

fn traverse_map_p1<'a, 'b>(map: &'b HashMap<&'a str, Vec<&'a str>>) -> Vec<Vec<&'a str>> {
    let mut deq: VecDeque<(&str, Vec<&str>)> = VecDeque::from([("start", vec!["start"])]);
    let mut paths: Vec<Vec<&str>> = Vec::new();

    while let Some((id, path)) = deq.pop_front() {
        let neighbors = map.get(id).expect("node does not exist in map");
        for &neighbor in neighbors {
            if is_large_cave(neighbor) || !path.contains(&neighbor) {
                let mut new_path = path.clone();
                new_path.push(neighbor);
                if neighbor == "end" {
                    paths.push(new_path);
                } else {
                    deq.push_back((neighbor, new_path))
                }
            }
        }
    }

    paths
}

fn traverse_map_p2<'a, 'b>(map: &'b HashMap<&'a str, Vec<&'a str>>) -> Vec<Vec<&'a str>> {
    let mut deq: VecDeque<(&str, Vec<&str>)> = VecDeque::from([("start", vec!["start"])]);
    let mut paths: Vec<Vec<&str>> = Vec::new();

    while let Some((id, path)) = deq.pop_front() {
        let neighbors = map.get(id).expect("node does not exist in map");
        for &neighbor in neighbors {
            let visited: HashSet<&str> = path
                .iter()
                .copied()
                .filter(|&id| !is_large_cave(id))
                .collect();
            if visited.len()
                < path
                    .iter()
                    .copied()
                    .filter(|&id| !is_large_cave(id))
                    .count()
            {
                if is_large_cave(neighbor) || !path.contains(&neighbor) {
                    let mut new_path = path.clone();
                    new_path.push(neighbor);
                    if neighbor == "end" {
                        paths.push(new_path);
                    } else {
                        deq.push_back((neighbor, new_path))
                    }
                }
            } else if neighbor != "start" {
                let mut new_path = path.clone();
                new_path.push(neighbor);
                if neighbor == "end" {
                    paths.push(new_path);
                } else {
                    deq.push_back((neighbor, new_path))
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
    println!("Part 1: {:?}", paths_p1.len());

    let paths_p2 = traverse_map_p2(&map);
    println!("Part 2: {:?}", paths_p2.len());
}
