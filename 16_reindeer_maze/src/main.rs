use std::{
    collections::{BinaryHeap, HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Error},
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Hash)]
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

#[derive(Clone, PartialEq, Eq, Debug)]
struct Point {
    pos: (usize, usize),
    enter_direction: Direction,
    cost: usize,
    previous_points: HashSet<(usize, usize)>,
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

fn add_to_queue(
    map: &[Vec<char>],
    queue: &mut BinaryHeap<Point>,
    point: &Point,
    direction: &Direction,
    cost_increase: usize,
) {
    if let Some(new_pos) = walk_direction(point.pos, direction) {
        if let Some('.' | 'E') = get_on_map(map, new_pos) {
            let cost = point.cost + cost_increase;

            if point.previous_points.contains(&new_pos) {
                return;
            }

            let mut path = point.previous_points.clone();
            path.insert(point.pos);

            queue.push(Point {
                pos: new_pos,
                cost,
                enter_direction: *direction,
                previous_points: path,
            });
        }
    }
}

fn get_min_cost_tiles(
    map: &[Vec<char>],
    start_pos: (usize, usize),
    end_pos: (usize, usize),
) -> usize {
    let mut queue: BinaryHeap<Point> = BinaryHeap::new();
    let mut visited_points: HashMap<(usize, usize, Direction), usize> = HashMap::new();

    queue.push(Point {
        pos: start_pos,
        enter_direction: Direction::Right,
        cost: 0,
        previous_points: HashSet::new(),
    });

    let mut paths_to_finish: Vec<(usize, HashSet<(usize, usize)>)> = Vec::new();

    while let Some(point) = queue.pop() {
        if point.pos == end_pos {
            let cost = point.cost;

            let mut path = point.previous_points.clone();
            path.insert(point.pos);

            paths_to_finish.push((cost, path));
            continue;
        }

        if point.cost
            > *visited_points
                .get(&(point.pos.0, point.pos.1, point.enter_direction))
                .unwrap_or(&usize::MAX)
        {
            continue;
        }

        visited_points.insert(
            (point.pos.0, point.pos.1, point.enter_direction),
            point.cost,
        );

        [
            (point.enter_direction, 1),
            (point.enter_direction.turn_clockwise(), 1001),
            (point.enter_direction.turn_counter_clockwise(), 1001),
        ]
        .iter()
        .for_each(|(direction, cost_increase)| {
            add_to_queue(map, &mut queue, &point, direction, *cost_increase)
        });
    }

    let min_cost = paths_to_finish
        .iter()
        .map(|path| path.0)
        .min()
        .expect("at least one path");

    let mut tiles_on_min_path: HashSet<(usize, usize)> = HashSet::new();

    paths_to_finish
        .iter()
        .filter(|path| path.0 == min_cost)
        .flat_map(|path| path.1.iter())
        .for_each(|pos| {
            tiles_on_min_path.insert(*pos);
        });

    tiles_on_min_path.len()
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

    let min_cost_tiles = get_min_cost_tiles(&map, start_pos, end_pos);

    println!("Min cost tiles is {min_cost_tiles}");

    Ok(())
}
