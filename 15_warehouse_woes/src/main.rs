use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    Box,
    Empty,
    Robot,
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_idx_diff(&self) -> (isize, isize) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }
}

impl From<char> for Direction {
    fn from(value: char) -> Self {
        match value {
            '<' => Direction::Left,
            '^' => Direction::Up,
            'v' => Direction::Down,
            '>' => Direction::Right,
            c => panic!("Unknown move {c}"),
        }
    }
}

fn parse_map(lines: &[String]) -> Vec<Vec<Tile>> {
    lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '#' => Tile::Wall,
                    '.' => Tile::Empty,
                    'O' => Tile::Box,
                    '@' => Tile::Robot,
                    c => panic!("Unknown tile {c}"),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn parse_moves(lines: &[String]) -> Vec<Direction> {
    lines
        .iter()
        .flat_map(|line| line.chars().map(|c| c.into()))
        .collect::<Vec<_>>()
}

fn get_next_pos(pos: (usize, usize), direction: &Direction) -> Option<(usize, usize)> {
    let idx_diff = direction.to_idx_diff();
    let new_y = pos.0.checked_add_signed(idx_diff.0)?;
    let new_x = pos.1.checked_add_signed(idx_diff.1)?;

    Some((new_y, new_x))
}

fn try_move(
    pos: (usize, usize),
    direction: &Direction,
    map: &mut Vec<Vec<Tile>>,
) -> Option<(usize, usize)> {
    let next_pos = get_next_pos(pos, direction)?;

    let next_tile = map.get(next_pos.0).and_then(|line| line.get(next_pos.1))?;

    let move_works = match next_tile {
        Tile::Wall => false,
        Tile::Robot => panic!("Tried pushing robot"),
        Tile::Box => try_move(next_pos, direction, map).is_some(),
        Tile::Empty => true,
    };

    if !move_works {
        return None;
    }

    assert!(matches!(map[next_pos.0][next_pos.1], Tile::Empty));

    map[next_pos.0][next_pos.1] = map[pos.0][pos.1].clone();
    map[pos.0][pos.1] = Tile::Empty;

    Some(next_pos)
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .map(|line| line.expect("valid line"))
        .collect::<Vec<_>>();

    let mut inputs = lines.split(|line| line.is_empty());

    let map = inputs.next().expect("map is present");
    let mut map = parse_map(map);

    let moves = inputs.next().expect("moves are present");
    let moves = parse_moves(moves);

    let mut robot_pos = map
        .iter()
        .enumerate()
        .flat_map(|(line_idx, line)| {
            line.iter()
                .enumerate()
                .map(move |(col_idx, item)| (line_idx, col_idx, item))
        })
        .find(|&(_, _, item)| item == &Tile::Robot)
        .map(|(line_idx, col_idx, _)| (line_idx, col_idx))
        .expect("robot is present");

    for direction in moves {
        robot_pos = try_move(robot_pos, &direction, &mut map).unwrap_or(robot_pos);
    }

    let box_pos_sum: usize = map
        .iter()
        .enumerate()
        .flat_map(|(line_idx, line)| {
            line.iter()
                .enumerate()
                .map(move |(col_idx, tile)| match tile {
                    Tile::Box => line_idx * 100 + col_idx,
                    _ => 0,
                })
        })
        .sum();

    println!("Box pos sum is {box_pos_sum}");

    Ok(())
}
