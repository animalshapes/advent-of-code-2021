#[derive(Debug, Clone)]
enum Number {
    Leaf(u8),
    Parent {
        left: Box<Number>,
        right: Box<Number>,
    },
}

impl Number {
    fn from_str(input: &str) -> Number {
        if input.len() == 1 {
            Number::Leaf(input.parse().unwrap())
        } else {
            let mut depth = 0;
            let mut split = 0usize;
            for (index, ele) in input.chars().enumerate() {
                match ele {
                    '[' => depth += 1,
                    ']' => depth -= 1,
                    ',' => {
                        if depth == 1 {
                            split = index;
                            break;
                        }
                    }
                    _ => (),
                }
            }
            let (left, right) = input.split_at(split);

            Number::Parent {
                left: Box::new(Number::from_str(&left[1..])),
                right: Box::new(Number::from_str(&right[1..right.len() - 1])),
            }
        }
    }

    fn magnitude(&self) -> usize {
        match self {
            Number::Leaf(value) => *value as usize,
            Number::Parent { left, right } => 3 * left.magnitude() + 2 * right.magnitude(),
        }
    }

    fn absorb_left(&mut self, val: u8) {
        match self {
            Number::Leaf(value) => *value += val,
            Number::Parent { left, right: _ } => left.absorb_left(val),
        }
    }

    fn absorb_right(&mut self, val: u8) {
        match self {
            Number::Leaf(value) => *value += val,
            Number::Parent { left: _, right } => right.absorb_right(val),
        }
    }

    fn split(&mut self) -> bool {
        match self {
            Number::Leaf(value) => {
                if *value >= 10 {
                    *self = Number::Parent {
                        left: Box::new(Number::Leaf(*value / 2)),
                        right: Box::new(Number::Leaf((*value + 1) / 2)),
                    };
                    true
                } else {
                    false
                }
            }
            Number::Parent { left, right } => left.split() || right.split(),
        }
    }

    fn add(first: &Number, second: &Number) -> Number {
        let mut parent = Number::Parent {
            left: Box::new(first.clone()),
            right: Box::new(second.clone()),
        };

        while parent.reduce(0).is_some() || parent.split() {}
        parent
    }

    fn reduce(&mut self, depth: u32) -> Option<(u8, u8)> {
        match self {
            Number::Leaf(_) => None,
            Number::Parent { left, right } => {
                if depth == 4 {
                    match (*left.clone(), *right.clone()) {
                        (Number::Leaf(l_value), Number::Leaf(r_value)) => {
                            *self = Number::Leaf(0);
                            Some((l_value, r_value))
                        }
                        (_, _) => None,
                    }
                } else if let Some((a, b)) = left.reduce(depth + 1) {
                    right.absorb_left(b);
                    Some((a, 0))
                } else if let Some((a, b)) = right.reduce(depth + 1) {
                    left.absorb_right(a);
                    Some((0, b))
                } else {
                    None
                }
            }
        }
    }
}

fn main() {
    let contents = include_str!("day18.txt").trim_end();

    let numbers: Vec<Number> = contents.lines().map(Number::from_str).collect();

    let mut total = numbers[0].clone();
    for number in numbers[1..].iter() {
        total = Number::add(&total, number);
    }

    println!("Part 1: {:?}", total.magnitude());

    let mut max = 0;
    for (index, number1) in numbers.iter().enumerate() {
        for number2 in numbers
            .iter()
            .take(index)
            .chain(numbers.iter().skip(index + 1))
        {
            let sum = Number::add(number1, number2).magnitude();
            max = max.max(sum);
        }
    }

    println!("Part 2: {:?}", max);
}
