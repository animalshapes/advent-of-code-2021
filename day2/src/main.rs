use std::{env, fs};

enum Action {
    Forward(i32),
    Down(i32),
    Up(i32),
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
    fs::read_to_string(filename).unwrap()
}

fn convert_to_actions(contents: String) -> Vec<Action> {
    let actions: Vec<Action> = contents
        .split('\n')
        .map(|s| {
            let info: Vec<&str> = s.split(' ').collect();
            let action = info[0];
            let value = info[1].parse::<i32>().unwrap();

            match action {
                "forward" => Action::Forward(value),
                "down" => Action::Down(value),
                "up" => Action::Up(value),
                _ => panic!("Unknown action"),
            }
        })
        .collect();
    actions
}

fn process_actions_q1(start: Location, actions: &[Action]) -> Location {
    let end: Location = actions
        .iter()
        .fold(start, |location, action| match action {
            Action::Forward(d) => Location {
                x: location.x + d,
                y: location.y,
            },
            Action::Down(d) => Location {
                x: location.x,
                y: location.y + d,
            },
            Action::Up(d) => Location {
                x: location.x,
                y: location.y - d,
            },
        });
    end
}

fn process_actions_q2(start: Status, actions: &[Action]) -> Status {
    let end: Status = actions
        .iter()
        .fold(start, |status, action| match action {
            Action::Forward(d) => Status {
                x: status.x + d,
                y: status.y + status.aim * d,
                aim: status.aim,
            },
            Action::Down(d) => Status {
                x: status.x,
                y: status.y,
                aim: status.aim + d,
            },
            Action::Up(d) => Status {
                x: status.x,
                y: status.y,
                aim: status.aim - d,
            },
        });
    end
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
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
