use std::{cmp, env, fs};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Point {
    x: usize,
    y: usize,
}

impl Point {
    fn from_str(input: &str) -> Point {
        let no_whitespace: String = input.chars().filter(|c| !c.is_whitespace()).collect();
        let values: Vec<usize> = no_whitespace
            .split(',')
            .map(|val| val.parse::<usize>().unwrap())
            .collect();
        Point {
            x: values[0],
            y: values[1],
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Line {
    start: Point,
    finish: Point,
}

impl Line {
    fn from_str(input: &str) -> Line {
        let points: Vec<Point> = input.split("->").map(|val| Point::from_str(val)).collect();
        if points[0].x <= points[1].x {
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

    fn get_points(&self) -> Vec<Point> {
        if self.is_horizontal() {
            (self.start.x..=self.finish.x)
                .map(|x| Point { x, y: self.start.y })
                .collect::<Vec<Point>>()
        } else if self.is_vertical() {
            let y_start = cmp::min(self.start.y, self.finish.y);
            let y_end = cmp::max(self.start.y, self.finish.y);

            (y_start..=y_end)
                .map(|y| Point { x: self.start.x, y })
                .collect::<Vec<Point>>()
        } else {
            let x_iter = self.start.x..=self.finish.x;
            if self.start.y < self.finish.y {
                let y_iter = self.start.y..=self.finish.y;
                x_iter
                    .zip(y_iter)
                    .map(|(x, y)| Point { x, y })
                    .collect::<Vec<Point>>()
            } else {
                let y_iter = (self.finish.y..=self.start.y).rev();
                x_iter
                    .zip(y_iter)
                    .map(|(x, y)| Point { x, y })
                    .collect::<Vec<Point>>()
            }
        }
    }
}

fn count_intersections(arr: &[Vec<i32>]) -> usize {
    arr.iter()
        .map(|row| row.iter().filter(|&ele| *ele > 1).count())
        .sum::<usize>()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let contents = fs::read_to_string(filename).unwrap();

    let mut arr = vec![vec![0; 1000]; 1000];

    let lines: Vec<Line> = contents
        .split('\n')
        .map(|row| Line::from_str(row))
        .collect();

    let (gridlines, diagonals): (Vec<Line>, Vec<Line>) = lines
        .iter()
        .partition(|&line| line.is_horizontal() || line.is_vertical());

    for line in gridlines {
        let points = line.get_points();
        for Point { x, y } in points {
            arr[x][y] += 1;
        }
    }

    let intersections1 = count_intersections(&arr);
    println!("Part 1: {:?}", intersections1);

    for line in diagonals {
        let points = line.get_points();
        for Point { x, y } in points {
            arr[x][y] += 1;
        }
    }

    let intersections2 = count_intersections(&arr);
    println!("Part 2: {:?}", intersections2);
}
