use core::panic;
use std::{
    collections::{BinaryHeap, HashMap},
    fs::File,
    io::{BufRead, BufReader, Error},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn_clockwise(self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    fn turn_counter_clockwise(self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    fn to_idx_diff(self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

#[derive(PartialEq, Eq, Debug)]
struct Point {
    pos: (usize, usize),
    enter_direction: Direction,
    cost: usize,
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn walk_direction(pos: (usize, usize), direction: &Direction) -> Option<(usize, usize)> {
    let idx_diff = direction.to_idx_diff();

    let new_x = pos.0.checked_add_signed(idx_diff.0)?;
    let new_y = pos.1.checked_add_signed(idx_diff.1)?;

    Some((new_x, new_y))
}

fn get_on_map(map: &[Vec<char>], pos: (usize, usize)) -> Option<char> {
    map.get(pos.0).and_then(|line| line.get(pos.1)).copied()
}

fn find_coordinates(map: &[Vec<char>], target: char) -> Option<(usize, usize)> {
    map.iter()
        .enumerate()
        .flat_map(|(line_idx, line)| {
            line.iter()
                .enumerate()
                .map(move |(col_idx, item)| (line_idx, col_idx, item))
        })
        .find(|&(_, _, item)| *item == target)
        .map(|(line_idx, col_idx, _)| (line_idx, col_idx))
}

fn insert_to_cost_if_cheaper(
    map: &[Vec<char>],
    costs: &mut HashMap<(usize, usize), usize>,
    queue: &mut BinaryHeap<Point>,
    point: &Point,
    direction: &Direction,
    cost_increase: usize,
) {
    if let Some(pos_forwards) = walk_direction(point.pos, direction) {
        if let Some('.' | 'E') = get_on_map(map, pos_forwards) {
            let cost = point.cost + cost_increase;

            if cost < *costs.get(&pos_forwards).unwrap_or(&usize::MAX) {
                costs.insert(pos_forwards, cost);
                queue.push(Point {
                    pos: pos_forwards,
                    cost,
                    enter_direction: *direction,
                });
            }
        }
    }
}

fn get_min_cost(map: &[Vec<char>], start_pos: (usize, usize), end_pos: (usize, usize)) -> usize {
    let mut costs: HashMap<(usize, usize), usize> = HashMap::new();
    let mut queue: BinaryHeap<Point> = BinaryHeap::new();

    queue.push(Point {
        pos: start_pos,
        enter_direction: Direction::Right,
        cost: 0,
    });

    while let Some(point) = queue.pop() {
        if point.pos == end_pos {
            return point.cost;
        }

        [
            (point.enter_direction, 1),
            (point.enter_direction.turn_clockwise(), 1001),
            (point.enter_direction.turn_counter_clockwise(), 1001),
        ]
        .iter()
        .for_each(|(direction, cost_increase)| {
            insert_to_cost_if_cheaper(
                map,
                &mut costs,
                &mut queue,
                &point,
                direction,
                *cost_increase,
            )
        });
    }

    panic!("no path could be found")
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let map = reader
        .lines()
        .map(|line| line.expect("valid line").chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let start_pos = find_coordinates(&map, 'S').expect("start is present");
    let end_pos = find_coordinates(&map, 'E').expect("end is present");

    let min_cost = get_min_cost(&map, start_pos, end_pos);

    println!("Min cost is {min_cost}");

    Ok(())
}
