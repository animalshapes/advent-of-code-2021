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

struct Status {
    x: i32,
    y: i32,
    aim: i32,
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

fn process_actions(start: Status, actions: Vec<Action>) -> Status {
    let end: Status = actions
        .iter()
        .fold(start, |status, action| match action.direction {
            Direction::Forward => Status {
                x: status.x + action.distance,
                y: status.y + status.aim * action.distance,
                aim: status.aim,
            },
            Direction::Down => Status {
                x: status.x,
                y: status.y,
                aim: status.aim + action.distance,
            },
            Direction::Up => Status {
                x: status.x,
                y: status.y,
                aim: status.aim - action.distance,
            },
        });
    return end;
}

fn main() {
    let filename: &str = "input.txt";
    let contents: String = read_file(filename);

    let actions = convert_to_actions(contents);
    let start = Status { x: 0, y: 0, aim: 0 };
    let end = process_actions(start, actions);

    let product = end.x * end.y;

    println!("{}", product);
}
