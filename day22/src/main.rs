use itertools::Itertools;
use std::iter;

#[derive(Debug)]
enum State {
    On,
    Off,
}

impl State {
    fn from_str(input: &str) -> State {
        match input {
            "on" => State::On,
            "off" => State::Off,
            _ => panic!("improper input"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Interval {
    start: i64,
    end: i64,
}

impl Interval {
    fn intersection(&self, other: &Interval) -> Option<Interval> {
        if self.start > other.end || other.start > self.end {
            None
        } else {
            Some(Interval {
                start: self.start.max(other.start),
                end: self.end.min(other.end),
            })
        }
    }

    fn subtract(&self, other: &Interval) -> Option<Vec<Interval>> {
        match self.intersection(other) {
            None => Some(vec![*self]),
            Some(Interval { start, end }) => {
                if start == self.start && end == self.end {
                    None
                } else if start == self.start {
                    Some(vec![Interval {
                        start: end + 1,
                        end: self.end,
                    }])
                } else if end == self.end {
                    Some(vec![Interval {
                        start: self.start,
                        end: start - 1,
                    }])
                } else {
                    Some(vec![
                        Interval {
                            start: self.start,
                            end: start - 1,
                        },
                        Interval {
                            start: end + 1,
                            end: self.end,
                        },
                    ])
                }
            }
        }
    }

    fn size(&self) -> i64 {
        self.end - self.start + 1
    }
}

#[derive(Debug, Clone, Copy)]
struct Volume {
    x: Interval,
    y: Interval,
    z: Interval,
}

impl Volume {
    fn intersection(&self, other: &Volume) -> Option<Volume> {
        if let (Some(x), Some(y), Some(z)) = (
            self.x.intersection(&other.x),
            self.y.intersection(&other.y),
            self.z.intersection(&other.z),
        ) {
            Some(Volume { x, y, z })
        } else {
            None
        }
    }

    fn subtract(&self, other: &Volume) -> Option<Vec<Volume>> {
        match self.intersection(other) {
            None => Some(vec![*self]),
            Some(Volume {
                x: x_inter,
                y: y_inter,
                z: z_inter,
            }) => {
                match (
                    self.x.subtract(&x_inter),
                    self.y.subtract(&y_inter),
                    self.z.subtract(&z_inter),
                ) {
                    (None, None, None) => None,
                    (Some(x_diff), None, None) => Some(
                        x_diff
                            .iter()
                            .map(|interval| Volume {
                                x: *interval,
                                y: self.y,
                                z: self.z,
                            })
                            .collect(),
                    ),
                    (None, Some(y_diff), None) => Some(
                        y_diff
                            .iter()
                            .map(|interval| Volume {
                                x: self.x,
                                y: *interval,
                                z: self.z,
                            })
                            .collect(),
                    ),
                    (None, None, Some(z_diff)) => Some(
                        z_diff
                            .iter()
                            .map(|interval| Volume {
                                x: self.x,
                                y: self.y,
                                z: *interval,
                            })
                            .collect(),
                    ),
                    (Some(x_diff), Some(y_diff), None) => Some(
                        x_diff
                            .iter()
                            .cartesian_product(y_diff.iter())
                            .chain(x_diff.iter().cartesian_product(iter::once(&y_inter)))
                            .chain(iter::once(&x_inter).cartesian_product(y_diff.iter()))
                            .map(|(&x, &y)| Volume { x, y, z: self.z })
                            .collect(),
                    ),
                    (Some(x_diff), None, Some(z_diff)) => Some(
                        x_diff
                            .iter()
                            .cartesian_product(z_diff.iter())
                            .chain(x_diff.iter().cartesian_product(iter::once(&z_inter)))
                            .chain(iter::once(&x_inter).cartesian_product(z_diff.iter()))
                            .map(|(&x, &z)| Volume { x, y: self.y, z })
                            .collect(),
                    ),
                    (None, Some(y_diff), Some(z_diff)) => Some(
                        y_diff
                            .iter()
                            .cartesian_product(z_diff.iter())
                            .chain(y_diff.iter().cartesian_product(iter::once(&z_inter)))
                            .chain(iter::once(&y_inter).cartesian_product(z_diff.iter()))
                            .map(|(&y, &z)| Volume { x: self.x, y, z })
                            .collect(),
                    ),
                    (Some(x_diff), Some(y_diff), Some(z_diff)) => Some(
                        x_diff
                            .iter()
                            .cartesian_product(y_diff.iter())
                            .cartesian_product(z_diff.iter())
                            .chain(
                                x_diff
                                    .iter()
                                    .cartesian_product(y_diff.iter())
                                    .cartesian_product(iter::once(&z_inter)),
                            )
                            .chain(
                                x_diff
                                    .iter()
                                    .cartesian_product(iter::once(&y_inter))
                                    .cartesian_product(z_diff.iter()),
                            )
                            .chain(
                                iter::once(&x_inter)
                                    .cartesian_product(y_diff.iter())
                                    .cartesian_product(z_diff.iter()),
                            )
                            .chain(
                                x_diff
                                    .iter()
                                    .cartesian_product(iter::once(&y_inter))
                                    .cartesian_product(iter::once(&z_inter)),
                            )
                            .chain(
                                iter::once(&x_inter)
                                    .cartesian_product(iter::once(&y_inter))
                                    .cartesian_product(z_diff.iter()),
                            )
                            .chain(
                                iter::once(&x_inter)
                                    .cartesian_product(y_diff.iter())
                                    .cartesian_product(iter::once(&z_inter)),
                            )
                            .map(|((&x, &y), &z)| Volume { x, y, z })
                            .collect(),
                    ),
                }
            }
        }
    }

    fn size(&self) -> i64 {
        self.x.size() * self.y.size() * self.z.size()
    }
}

#[derive(Debug)]
struct Step {
    state: State,
    volume: Volume,
}

impl Step {
    fn from_str(input: &str) -> Step {
        let (state_raw, intervals_raw) = input.split_once(' ').unwrap();
        let state = State::from_str(state_raw);
        let mut intervals = intervals_raw.split(',').map(|entry| {
            let (start, end) = entry[2..].split_once("..").unwrap();
            Interval {
                start: start.parse::<i64>().unwrap(),
                end: end.parse::<i64>().unwrap(),
            }
        });

        let volume = Volume {
            x: intervals.next().unwrap(),
            y: intervals.next().unwrap(),
            z: intervals.next().unwrap(),
        };

        Step { state, volume }
    }
}

fn main() {
    let contents = include_str!("day22.txt").trim_end();

    let steps: Vec<Step> = contents.lines().map(Step::from_str).collect();

    let mut on: Vec<Volume> = Vec::new();

    for (index, step) in steps.iter().enumerate() {
        match step.state {
            State::On => {
                let mut current_volumes = vec![step.volume];
                for volume in on.iter() {
                    current_volumes = current_volumes
                        .iter()
                        .flat_map(|ele| match ele.subtract(volume) {
                            None => vec![],
                            Some(x) => x,
                        })
                        .collect();
                }
                on.append(&mut current_volumes);
            }
            State::Off => {
                on = on
                    .iter()
                    .flat_map(|ele| match ele.subtract(&step.volume) {
                        None => vec![],
                        Some(x) => x,
                    })
                    .collect()
            }
        }

        if index == 19 {
            let p1_total: i64 = on.iter().map(|ele| ele.size()).sum();
            println!("Part 1: {}", p1_total);
        }
    }

    let p2_total: i64 = on.iter().map(|ele| ele.size()).sum();

    println!("Part 2: {}", p2_total);
}
