use std::fs;

enum Direction {
    Forward,
    Down,
    Up,
}

struct Action {
    direction: Direction,
    distance: i32,
}

struct Location {
    x: i32,
    y: i32,
}

fn read_file(filename: &str) -> String {
    return fs::read_to_string(filename).unwrap();
}

fn convert_to_actions(contents: String) -> Vec<Action> {
    let actions: Vec<Action> = contents
        .split("\n")
        .map(|s| {
            let info: Vec<&str> = s.split(" ").collect();
            let action = info[0];
            let value = info[1].parse::<i32>().unwrap();

            match action {
                "forward" => Action {
                    direction: Direction::Forward,
                    distance: value,
                },
                "down" => Action {
                    direction: Direction::Down,
                    distance: value,
                },
                "up" => Action {
                    direction: Direction::Up,
                    distance: value,
                },
                _ => panic!("Unknown action"),
            }
        })
        .collect();
    return actions;
}

fn process_actions(start: Location, actions: Vec<Action>) -> Location {
    let end: Location = actions
        .iter()
        .fold(start, |location, action| match action.direction {
            Direction::Forward => Location {
                x: location.x + action.distance,
                y: location.y,
            },
            Direction::Down => Location {
                x: location.x,
                y: location.y + action.distance,
            },
            Direction::Up => Location {
                x: location.x,
                y: location.y - action.distance,
            },
        });
    return end;
}

fn main() {
    let filename: &str = "input.txt";
    let contents: String = read_file(filename);

    let actions = convert_to_actions(contents);
    let start = Location { x: 0, y: 0 };
    let end = process_actions(start, actions);

    let product = end.x * end.y;

    println!("{}", product);
}
