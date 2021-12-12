use std::collections::{HashMap, VecDeque};
use std::rc::Rc;

type Link<T> = Option<Rc<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

struct LinkedList<T> {
    head: Link<T>,
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        LinkedList { head: None }
    }

    pub fn prepend(&self, elem: T) -> LinkedList<T> {
        match &self.head {
            Some(node) => LinkedList {
                head: Some(Rc::new(Node {
                    elem: elem,
                    next: Some(Rc::clone(&node)),
                })),
            },
            None => LinkedList {
                head: Some(Rc::new(Node {
                    elem: elem,
                    next: None,
                })),
            },
        }
    }

    pub fn iter(&self) -> Iter<'_, T> {
        Iter {
            next: self.head.as_deref(),
        }
    }
}

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            &node.elem
        })
    }
}

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
    let initial_list = LinkedList::new().prepend("start");
    let mut deq: VecDeque<(&str, LinkedList<&str>)> = VecDeque::from([("start", initial_list)]);
    let mut paths: i32 = 0;

    while let Some((id, path)) = deq.pop_front() {
        let neighbors = map.get(id).expect("node does not exist in map");
        for &neighbor in neighbors {
            if is_large_cave(neighbor) || !path.iter().any(|&ele| ele == neighbor) {
                if neighbor == "end" {
                    paths += 1
                } else {
                    let new_path = path.prepend(neighbor);
                    deq.push_back((neighbor, new_path))
                }
            }
        }
    }

    paths
}

fn traverse_map_p2<'a, 'b>(map: &'b HashMap<&'a str, Vec<&'a str>>) -> i32 {
    let initial_list = LinkedList::new().prepend("start");
    let mut deq: VecDeque<(&str, LinkedList<&str>, bool)> =
        VecDeque::from([("start", initial_list, false)]);
    let mut paths: i32 = 0;

    while let Some((id, path, dupe)) = deq.pop_front() {
        let neighbors = map.get(id).expect("node does not exist in map");
        for &neighbor in neighbors {
            if dupe {
                if is_large_cave(neighbor) || !path.iter().any(|&ele| ele == neighbor) {
                    if neighbor == "end" {
                        paths += 1;
                    } else {
                        let new_path = path.prepend(neighbor);
                        deq.push_back((neighbor, new_path, dupe))
                    }
                }
            } else if neighbor != "start" {
                if neighbor == "end" {
                    paths += 1;
                } else {
                    let new_path = path.prepend(neighbor);
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
