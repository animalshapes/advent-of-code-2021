#[derive(Debug)]
enum Operation {
    Sum,
    Product,
    Minimum,
    Maximum,
    GreaterThan,
    LessThan,
    Equal,
}

impl Operation {
    fn from_value(input: u64) -> Operation {
        match input {
            0 => Operation::Sum,
            1 => Operation::Product,
            2 => Operation::Minimum,
            3 => Operation::Maximum,
            5 => Operation::GreaterThan,
            6 => Operation::LessThan,
            7 => Operation::Equal,
            _ => panic!("unexpected operation"),
        }
    }
}

#[derive(Debug)]
struct ValuePacket {
    version: u64,
    value: u64,
}

#[derive(Debug)]
struct OperationPacket {
    version: u64,
    operation: Operation,
    children: Vec<Box<dyn Packet>>,
}

trait Packet: std::fmt::Debug {
    fn evaluate(&self) -> u64;

    fn version_total(&self) -> u64;
}

impl Packet for ValuePacket {
    fn evaluate(&self) -> u64 {
        self.value
    }

    fn version_total(&self) -> u64 {
        self.version
    }
}

impl Packet for OperationPacket {
    fn evaluate(&self) -> u64 {
        match self.operation {
            Operation::Sum => self.children.iter().map(|ele| ele.evaluate()).sum(),
            Operation::Product => self.children.iter().map(|ele| ele.evaluate()).product(),
            Operation::Minimum => self
                .children
                .iter()
                .map(|ele| ele.evaluate())
                .min()
                .unwrap(),
            Operation::Maximum => self
                .children
                .iter()
                .map(|ele| ele.evaluate())
                .max()
                .unwrap(),
            Operation::GreaterThan => {
                let mut values = self.children.iter().map(|ele| ele.evaluate());
                let first = values.next().unwrap();
                let second = values.next().unwrap();
                (first > second) as u64
            }
            Operation::LessThan => {
                let mut values = self.children.iter().map(|ele| ele.evaluate());
                let first = values.next().unwrap();
                let second = values.next().unwrap();
                (first < second) as u64
            }
            Operation::Equal => {
                let mut values = self.children.iter().map(|ele| ele.evaluate());
                let first = values.next().unwrap();
                let second = values.next().unwrap();
                (first == second) as u64
            }
        }
    }

    fn version_total(&self) -> u64 {
        let child_totals: u64 = self
            .children
            .iter()
            .map(|child| child.version_total())
            .sum();
        self.version + child_totals
    }
}

fn hex_str_to_bin_str(hex: &str) -> String {
    hex.chars()
        .map(|ele| {
            format!(
                "{:04b}",
                u64::from_str_radix(&ele.to_string(), 16).expect("not true hex")
            )
        })
        .collect::<Vec<_>>()
        .concat()
}

fn parse_next_packet(input: &str) -> (Box<dyn Packet>, &str) {
    let version = u64::from_str_radix(input.get(0..3).unwrap(), 2).unwrap();
    let packet_type_id = u64::from_str_radix(input.get(3..6).unwrap(), 2).unwrap();

    match packet_type_id {
        4 => {
            let mut current_str: &str = input.get(6..).unwrap();
            let mut numbers: Vec<&str> = Vec::new();
            loop {
                let (left, right) = current_str.get(..5).unwrap().split_at(1);
                numbers.push(right);
                current_str = current_str.get(5..).unwrap();
                if left == "0" {
                    break;
                }
            }

            let packet = Box::new(ValuePacket {
                version,
                value: u64::from_str_radix(&numbers.concat(), 2).expect("invalid binary"),
            });

            (packet, current_str)
        }
        _ => {
            let length_type_id = input.chars().nth(6).unwrap();
            let operation = Operation::from_value(packet_type_id);
            match length_type_id {
                '0' => {
                    let children_num_bits =
                        usize::from_str_radix(input.get(7..22).unwrap(), 2).unwrap();
                    let subpackets_str = input.get(22..22 + children_num_bits).unwrap();
                    let children = parse_all_packets(subpackets_str);
                    let packet = Box::new(OperationPacket {
                        version,
                        operation,
                        children,
                    });
                    (packet, input.get(22 + children_num_bits..).unwrap())
                }
                '1' => {
                    let num_packets = usize::from_str_radix(input.get(7..18).unwrap(), 2).unwrap();

                    let mut children: Vec<Box<dyn Packet>> = Vec::new();
                    let mut next_packet = input.get(18..).unwrap();

                    for _ in 0..num_packets {
                        let output = parse_next_packet(next_packet);
                        children.push(output.0);
                        next_packet = output.1;
                    }

                    let packet = Box::new(OperationPacket {
                        version,
                        operation,
                        children,
                    });

                    (packet, next_packet)
                }
                _ => panic!("invalid length type id"),
            }
        }
    }
}

fn parse_all_packets(input: &str) -> Vec<Box<dyn Packet>> {
    let mut packets: Vec<Box<dyn Packet>> = Vec::new();

    let mut next_packet = input;
    while next_packet.contains('1') {
        let output = parse_next_packet(next_packet);
        packets.push(output.0);
        next_packet = output.1;
    }

    packets
}

fn main() {
    let contents = include_str!("day16.txt").trim_end();

    let bin = hex_str_to_bin_str(contents);

    let packet_list = parse_all_packets(&bin[..]);
    let packet = packet_list.first().unwrap();

    println!("Part 1: {:?}", packet.version_total());
    println!("Part 2: {:?}", packet.evaluate());
}
