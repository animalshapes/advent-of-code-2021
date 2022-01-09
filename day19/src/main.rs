use hashbrown::HashSet;

type Coordinate = (i32, i32, i32);

static ORIENTATIONS: [fn(Coordinate) -> Coordinate; 24] = [
    |(x, y, z)| (x, y, z),
    |(x, y, z)| (x, -z, y),
    |(x, y, z)| (x, -y, -z),
    |(x, y, z)| (x, z, -y),
    |(x, y, z)| (-x, -y, z),
    |(x, y, z)| (-x, z, y),
    |(x, y, z)| (-x, y, -z),
    |(x, y, z)| (-x, -z, -y),
    |(x, y, z)| (y, -x, z),
    |(x, y, z)| (y, z, x),
    |(x, y, z)| (y, x, -z),
    |(x, y, z)| (y, -z, -x),
    |(x, y, z)| (-y, x, z),
    |(x, y, z)| (-y, -z, x),
    |(x, y, z)| (-y, -x, -z),
    |(x, y, z)| (-y, z, -x),
    |(x, y, z)| (z, -y, x),
    |(x, y, z)| (z, -x, -y),
    |(x, y, z)| (z, y, -x),
    |(x, y, z)| (z, x, y),
    |(x, y, z)| (-z, -x, y),
    |(x, y, z)| (-z, y, x),
    |(x, y, z)| (-z, x, -y),
    |(x, y, z)| (-z, -y, -x),
];

fn merge_scan(full_map: &mut HashSet<Coordinate>, scan: &[Coordinate]) -> Option<Coordinate> {
    for orientation in ORIENTATIONS.iter() {
        let rotated: Vec<Coordinate> = scan.iter().map(|&ele| orientation(ele)).collect();
        for (x1, y1, z1) in full_map.iter() {
            for (x2, y2, z2) in rotated.iter() {
                let (dx, dy, dz) = (x1 - x2, y1 - y2, z1 - z2);
                let translated = rotated.iter().map(|(x, y, z)| (x + dx, y + dy, z + dz));
                if translated
                    .clone()
                    .filter(|coord| full_map.contains(coord))
                    .count()
                    >= 12
                {
                    full_map.extend(translated);
                    return Some((dx, dy, dz));
                }
            }
        }
    }

    None
}

fn main() {
    let contents = include_str!("day19.txt");
    let mut scans: Vec<Vec<Coordinate>> = contents
        .split("\n\n")
        .map(|scan| {
            scan.lines()
                .skip(1)
                .map(|line| {
                    let values: Vec<i32> =
                        line.split(',').map(|ele| ele.parse().unwrap()).collect();
                    (values[0], values[1], values[2])
                })
                .collect()
        })
        .collect();

    let mut full_map: HashSet<Coordinate> = scans.swap_remove(0).into_iter().collect();
    let mut transforms: Vec<Coordinate> = Vec::new();

    while !scans.is_empty() {
        let mut to_remove: Vec<usize> = Vec::new();
        for (index, scan) in scans.iter().enumerate() {
            if let Some(coord) = merge_scan(&mut full_map, scan) {
                to_remove.push(index);
                transforms.push(coord);
            }
        }
        while let Some(index) = to_remove.pop() {
            scans.swap_remove(index);
        }
    }

    println!("Part 1: {:?}", full_map.len());

    let mut max = 0;
    for (index, (x1, y1, z1)) in transforms.iter().enumerate() {
        for (x2, y2, z2) in transforms.iter().skip(index + 1) {
            max = max.max((x1 - x2).abs() + (y1 - y2).abs() + (z1 - z2).abs());
        }
    }

    println!("Part 2: {:?}", max);
}
