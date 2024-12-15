use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

#[derive(Clone, Debug, PartialEq, Eq)]
enum Tile {
    Wall,
    BoxLeft,
    BoxRight,
    Empty,
    Robot,
}

fn print_map(map: &[Vec<Tile>]) {
    map.iter()
        .map(|line| {
            line.iter()
                .map(|tile| match tile {
                    Tile::Wall => '#',
                    Tile::BoxLeft => '[',
                    Tile::BoxRight => ']',
                    Tile::Empty => '.',
                    Tile::Robot => '@',
                })
                .collect::<String>()
        })
        .for_each(|line| println!("{line}"));
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
                .flat_map(|c| match c {
                    '#' => [Tile::Wall, Tile::Wall],
                    '.' => [Tile::Empty, Tile::Empty],
                    'O' => [Tile::BoxLeft, Tile::BoxRight],
                    '@' => [Tile::Robot, Tile::Empty],
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

fn try_move_box(
    pos: (usize, usize),
    direction: &Direction,
    map: &[Vec<Tile>],
) -> Option<Vec<Vec<Tile>>> {
    let mut map_copy = map.to_owned();

    let tile = &map[pos.0][pos.1];

    match (tile, direction) {
        (Tile::BoxLeft, Direction::Up | Direction::Down) => {
            map_copy = try_move(pos, direction, &mut map_copy)?.2;
            map_copy = try_move((pos.0, pos.1 + 1), direction, &mut map_copy)?.2;
        }
        (Tile::BoxRight, Direction::Up | Direction::Down) => {
            map_copy = try_move(pos, direction, &mut map_copy)?.2;
            map_copy = try_move((pos.0, pos.1 - 1), direction, &mut map_copy)?.2;
        }
        (Tile::BoxRight, Direction::Left) => {
            map_copy = try_move((pos.0, pos.1 - 1), direction, &mut map_copy)?.2;
            map_copy = try_move(pos, direction, &mut map_copy)?.2;
        }
        (Tile::BoxLeft, Direction::Right) => {
            map_copy = try_move((pos.0, pos.1 + 1), direction, &mut map_copy)?.2;
            map_copy = try_move(pos, direction, &mut map_copy)?.2;
        }
        _ => panic!("Invalid tile passed to fn"),
    }

    Some(map_copy)
}

fn try_move(
    pos: (usize, usize),
    direction: &Direction,
    map: &mut [Vec<Tile>],
) -> Option<(usize, usize, Vec<Vec<Tile>>)> {
    let next_pos = get_next_pos(pos, direction)?;

    let next_tile = map.get(next_pos.0).and_then(|line| line.get(next_pos.1))?;

    let mut new_map: Option<Vec<Vec<Tile>>> = None;
    let move_works = match next_tile {
        Tile::Wall => false,
        Tile::Robot => panic!("Tried pushing robot"),
        Tile::BoxLeft | Tile::BoxRight => {
            new_map = Some(try_move_box(next_pos, direction, map)?);
            true
        }
        Tile::Empty => true,
    };

    let mut map = new_map.unwrap_or(map.to_owned());

    if !move_works {
        return None;
    }

    assert!(matches!(map[next_pos.0][next_pos.1], Tile::Empty));

    map[next_pos.0][next_pos.1] = map[pos.0][pos.1].clone();
    map[pos.0][pos.1] = Tile::Empty;

    Some((next_pos.0, next_pos.1, map))
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

    print_map(&map);
    for direction in moves {
        let (robot_y, robot_x, new_map) =
            try_move(robot_pos, &direction, &mut map).unwrap_or((robot_pos.0, robot_pos.1, map));

        robot_pos = (robot_y, robot_x);
        map = new_map;

        print_map(&map);
    }

    let box_pos_sum: usize = map
        .iter()
        .enumerate()
        .flat_map(|(line_idx, line)| {
            line.iter()
                .enumerate()
                .map(move |(col_idx, tile)| match tile {
                    Tile::BoxLeft => line_idx * 100 + col_idx,
                    _ => 0,
                })
        })
        .sum();

    println!("Box pos sum is {box_pos_sum}");

    Ok(())
}
