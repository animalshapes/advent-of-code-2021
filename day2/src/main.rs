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

fn process_actions_q1(start: Location, actions: &Vec<Action>) -> Location {
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

fn process_actions_q2(start: Status, actions: &Vec<Action>) -> Status {
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
    let filename: &str = "src/input.txt";
    let contents = read_file(filename);

    let actions = convert_to_actions(contents);

    let start_q1 = Location { x: 0, y: 0 };
    let end_q1 = process_actions_q1(start_q1, &actions);
    let q1_ans = end_q1.x * end_q1.y;
    println!("Part 1: {}", q1_ans);

    let start_q2 = Status { x: 0, y: 0, aim: 0 };
    let end_q2 = process_actions_q2(start_q2, &actions);
    let q2_ans = end_q2.x * end_q2.y;
    println!("Part 2: {}", q2_ans);
}
