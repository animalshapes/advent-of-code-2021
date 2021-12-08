use std::{env, fs};

#[derive(Copy, Clone, PartialEq, Eq)]
struct Point {
    x: u64,
    y: u64,
}

impl Point {
    fn from_str(input: &str) -> Point {
        let values: Vec<u64> = input
            .split(',')
            .map(|val| val.parse::<u64>().unwrap())
            .collect();
        Point {
            x: values[0],
            y: values[1],
        }
    }

    fn plus(&self, input: &Point) -> Point {
        Point {
            x: self.x + input.x,
            y: self.y + input.y,
        }
    }

    fn sub(&self, input: &Point) -> Point {
        Point {
            x: self.x - input.x,
            y: self.y - input.y,
        }
    }

    fn multiply(&self, factor: u64) -> Point {
        Point {
            x: self.x * factor,
            y: self.y * factor,
        }
    }

    fn cross(&self, input: &Point) -> u64 {
        self.x * input.y - self.y * input.x
    }
}

struct Line {
    start: Point,
    finish: Point,
}

impl Line {
    fn from_str(input: &str) -> Line {
        let points: Vec<Point> = input.split("->").map(|val| Point::from_str(val)).collect();
        if points[0].x + points[0].y <= points[1].x + points[1].y {
            Line {
                start: points[0],
                finish: points[1],
            }
        } else {
            Line {
                start: points[1],
                finish: points[0],
            }
        }
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.finish.x
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.finish.y
    }

    fn intersect(&self, line: &Line) -> Option<Point> {
        let p = self.start;
        let q = line.start;
        let r = self.finish.sub(&self.start);
        let s = line.finish.sub(&line.start);
        let t = q.sub(&p).cross(&s) / (r.cross(&s));
        let u = q.sub(&p).cross(&r) / (r.cross(&s));
        if (t >= 0.0) && (t <= 1.0) && (u >= 0.0) && (u <= 1.0) {
            return Some(p.plus(&r.multiply(t)))
        }
        None
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();
}
