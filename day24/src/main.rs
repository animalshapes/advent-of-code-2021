#[derive(Debug)]
enum Var {
    W,
    X,
    Y,
    Z,
}

impl Var {
    fn from_str(input: &str) -> Var {
        match input {
            "w" => Var::W,
            "x" => Var::X,
            "y" => Var::Y,
            "z" => Var::Z,
            _ => panic!("invalid variable"),
        }
    }

    fn index(&self) -> usize {
        match self {
            Var::W => 0,
            Var::X => 1,
            Var::Y => 2,
            Var::Z => 3,
        }
    }
}

#[derive(Debug)]
enum Value {
    Var(Var),
    Raw(i64),
}

impl Value {
    fn from_str(input: &str) -> Value {
        if let Ok(val) = input.parse() {
            Value::Raw(val)
        } else {
            Value::Var(Var::from_str(input))
        }
    }
}

#[derive(Debug)]
struct State {
    contents: [i64; 4],
}

impl State {
    fn new() -> State {
        State { contents: [0; 4] }
    }

    fn get_value(&self, value: &Value) -> i64 {
        match value {
            Value::Raw(value) => *value,
            Value::Var(other) => self.contents[other.index()],
        }
    }

    fn check_valid(&self) -> bool {
        self.contents[Var::Z.index()] == 0
    }
}

#[derive(Debug)]
enum Instr {
    Input(Var),
    Add(Var, Value),
    Mul(Var, Value),
    Div(Var, Value),
    Mod(Var, Value),
    Eql(Var, Value),
}

impl Instr {
    fn from_str(input: &str) -> Instr {
        let (instruction, params) = input.split_once(' ').unwrap();
        match instruction {
            "inp" => Instr::Input(Var::from_str(params)),
            "add" => {
                let (param1, param2) = params.split_once(' ').unwrap();
                Instr::Add(Var::from_str(param1), Value::from_str(param2))
            }
            "mul" => {
                let (param1, param2) = params.split_once(' ').unwrap();
                Instr::Mul(Var::from_str(param1), Value::from_str(param2))
            }
            "div" => {
                let (param1, param2) = params.split_once(' ').unwrap();
                Instr::Div(Var::from_str(param1), Value::from_str(param2))
            }
            "mod" => {
                let (param1, param2) = params.split_once(' ').unwrap();
                Instr::Mod(Var::from_str(param1), Value::from_str(param2))
            }
            "eql" => {
                let (param1, param2) = params.split_once(' ').unwrap();
                Instr::Eql(Var::from_str(param1), Value::from_str(param2))
            }
            _ => panic!("invalid instruction"),
        }
    }
}

fn execute_instructions(instructions: &[Instr], inputs: &[i64]) -> State {
    let mut inputs_iter = inputs.iter();
    let mut state: State = State::new();
    for instr in instructions.iter() {
        match instr {
            Instr::Input(var) => state.contents[var.index()] = *inputs_iter.next().unwrap(),
            Instr::Add(var, addend) => state.contents[var.index()] += state.get_value(addend),
            Instr::Mul(var, factor) => state.contents[var.index()] *= state.get_value(factor),
            Instr::Div(var, divisor) => state.contents[var.index()] /= state.get_value(divisor),
            Instr::Mod(var, modulo) => state.contents[var.index()] %= state.get_value(modulo),
            Instr::Eql(var, test) => {
                state.contents[var.index()] =
                    (state.contents[var.index()] == state.get_value(test)) as i64
            }
        }
    }

    state
}

fn main() {
    let contents = include_str!("day24.txt").trim_end();
    let instructions: Vec<Instr> = contents.lines().map(Instr::from_str).collect();

    let inputs_p1 = [9, 7, 9, 1, 9, 9, 9, 7, 2, 9, 9, 4, 9, 5];

    let state_p1 = execute_instructions(&instructions, &inputs_p1);

    if state_p1.check_valid() {
        let represented = inputs_p1.iter().fold(0, |acc, ele| acc * 10 + ele);
        println!("Part 1: {:?}", represented);
    }

    let inputs_p2 = [5, 1, 6, 1, 9, 1, 3, 1, 1, 8, 1, 1, 3, 1];

    let state_p2 = execute_instructions(&instructions, &inputs_p2);
    if state_p2.check_valid() {
        let represented = inputs_p2.iter().fold(0, |acc, ele| acc * 10 + ele);
        println!("Part 2: {:?}", represented);
    }
}
