fn parse_bounds(raw_bounds: &str) -> Vec<i32> {
    raw_bounds
        .split('=')
        .last()
        .unwrap()
        .split("..")
        .map(|ele| ele.parse::<i32>().unwrap())
        .collect()
}

fn check_bounds(val: i32, bounds: &[i32]) -> bool {
    (val >= bounds[0]) && (val <= bounds[1])
}

fn move_positions(v_x: &mut i32, v_y: &mut i32, pos_x: &mut i32, pos_y: &mut i32) {
    *pos_x += *v_x;
    *pos_y += *v_y;
    *v_y -= 1;
    *v_x = if *v_x > 0 { *v_x - 1 } else { 0 }
}

fn main() {
    let contents = include_str!("day17.txt").trim_end();

    let mut bounds_raw = contents.split(": ").last().unwrap().split(", ");

    let x_bounds = parse_bounds(bounds_raw.next().unwrap());
    let y_bounds = parse_bounds(bounds_raw.next().unwrap());

    let mut valid_count = 0;
    let mut actual_max = 0;

    for x in 0..=x_bounds[1] {
        for y in y_bounds[0]..500 {
            let mut x_velocity = x;
            let mut y_velocity = y;
            let mut x_position = 0;
            let mut y_position = 0;

            let mut max_y_position = 0;
            let mut valid_flag = false;

            while y_position > y_bounds[0] {
                move_positions(
                    &mut x_velocity,
                    &mut y_velocity,
                    &mut x_position,
                    &mut y_position,
                );

                if y_position > max_y_position {
                    max_y_position = y_position;
                }

                if check_bounds(x_position, &x_bounds) && check_bounds(y_position, &y_bounds) {
                    valid_flag = true;
                    break;
                }
            }

            if valid_flag {
                valid_count += 1;
                if max_y_position > actual_max {
                    actual_max = max_y_position;
                }
            }
        }
    }

    println!("Part 1: {:?}", actual_max);
    println!("Part 2: {:?}", valid_count);
}
