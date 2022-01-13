use itertools::Itertools;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::collections::HashMap;

type Rooms<const N: usize> = [[u8; N]; 4];
type Corridor = [u8; 11];
type Game<const N: usize> = (Corridor, Rooms<N>);

fn correct<const N: usize>(rooms: &Rooms<N>) -> bool {
    rooms
        .iter()
        .zip("ABCD".bytes())
        .all(|(room, target)| room.iter().all(|ele| *ele == target))
}

fn make_move<const N: usize>(
    (mut corridor, mut rooms): Game<N>,
    corr_ind: usize,
    room_num: usize,
    room_depth: usize,
) -> (Game<N>, usize) {
    let piece = if corridor[corr_ind] == b'.' {
        rooms[room_num][room_depth]
    } else {
        corridor[corr_ind]
    } - b'A';
    let c0 = [2, 4, 6, 8][room_num];
    let energy =
        (room_depth + 1 + c0.max(corr_ind) - c0.min(corr_ind)) * [1, 10, 100, 1000][piece as usize];
    std::mem::swap(&mut corridor[corr_ind], &mut rooms[room_num][room_depth]);
    ((corridor, rooms), energy)
}

fn generate_states<const N: usize>(state: &Game<N>) -> Vec<(Game<N>, usize)> {
    let (corridor, rooms) = state;
    let mut states: Vec<(Game<N>, usize)> = Vec::new();

    for (corr_index, &corr_ele) in corridor.iter().enumerate() {
        if corr_ele == b'.' {
            continue;
        }
        let target_room = (corr_ele - b'A') as usize;
        let target_room_index = [2, 4, 6, 8][target_room];
        let (corridor_range_start, corridor_range_end) = if corr_index > target_room_index {
            (target_room_index, corr_index)
        } else {
            (corr_index + 1, target_room_index + 1)
        };

        if corridor
            .iter()
            .skip(corridor_range_start)
            .take(corridor_range_end - corridor_range_start)
            .any(|&ele| ele != b'.')
        {
            continue;
        }

        let first_empty_index = match rooms[target_room]
            .iter()
            .enumerate()
            .take_while(|(_, &ele)| ele == b'.')
            .last()
        {
            Some((index, _)) => index,
            None => continue,
        };

        if rooms[target_room]
            .iter()
            .skip(first_empty_index + 1)
            .any(|&room_ele| room_ele != corr_ele)
        {
            continue;
        }

        states.push(make_move(
            *state,
            corr_index,
            target_room,
            first_empty_index,
        ));
    }
    for (room_num, room) in rooms.iter().enumerate() {
        let first_occupied_index = match room.iter().enumerate().find(|(_, &ele)| ele != b'.') {
            Some((index, _)) => index,
            None => continue,
        };
        let target_room_index = [2, 4, 6, 8][room_num];
        let valid_moves = (target_room_index..corridor.len())
            .take_while(|&c| corridor[c] == b'.')
            .chain(
                (0..target_room_index)
                    .rev()
                    .take_while(|&c| corridor[c] == b'.'),
            )
            .filter(|c| ![2, 4, 6, 8].contains(c))
            .map(|c| make_move(*state, c, room_num, first_occupied_index));
        states.extend(valid_moves);
    }

    states
}

fn optimize<const N: usize>(state: Game<N>) -> usize {
    let mut heap: BinaryHeap<Reverse<(usize, Game<N>)>> = BinaryHeap::from([Reverse((0, state))]);
    let mut costs: HashMap<Game<N>, usize> = HashMap::new();
    while let Some(Reverse((cost, state))) = heap.pop() {
        if correct(&state.1) {
            return cost;
        }
        if let Some(cost_map) = costs.get(&state) {
            if cost > *cost_map {
                continue;
            }
        }

        for (new_state, energy) in generate_states(&state) {
            let new_cost = energy + cost;
            if new_cost < *costs.get(&new_state).unwrap_or(&usize::MAX) {
                costs.insert(new_state, new_cost);
                heap.push(Reverse((new_cost, new_state)));
            }
        }
    }
    0
}

fn main() {
    let contents = include_str!("day23.txt").trim_end();
    let (l1, l2) = contents
        .lines()
        .skip(2)
        .take(2)
        .map(|row| row.as_bytes())
        .collect_tuple()
        .unwrap();

    let part1 = [
        [l1[3], l2[3]],
        [l1[5], l2[5]],
        [l1[7], l2[7]],
        [l1[9], l2[9]],
    ];
    let part2 = [
        [l1[3], b'D', b'D', l2[3]],
        [l1[5], b'C', b'B', l2[5]],
        [l1[7], b'B', b'A', l2[7]],
        [l1[9], b'A', b'C', l2[9]],
    ];

    let corridor = [b'.'; 11];

    let p1 = optimize((corridor, part1));
    let p2 = optimize((corridor, part2));

    println!("Part 1: {:?}", p1);
    println!("Part 2: {:?}", p2);
}
