use core::panic;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Error},
};

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
            Direction::Right => (0, 1),
            Direction::Left => (0, -1),
        }
    }

    fn turn_right(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Right => Direction::Down,
            Direction::Left => Direction::Up,
        }
    }
}

fn walk_direction(current_pos: (usize, usize), direction: &Direction) -> Option<(usize, usize)> {
    let idx_diff = direction.to_idx_diff();

    let new_y = current_pos.0.checked_add_signed(idx_diff.0)?;
    let new_x = current_pos.1.checked_add_signed(idx_diff.1)?;

    Some((new_y, new_x))
}

fn count_path_tiles(starting_pos: (usize, usize), map: &[Vec<char>]) -> usize {
    let mut visited_tiles: HashSet<(usize, usize)> = HashSet::new();
    visited_tiles.insert(starting_pos);

    let mut current_pos = starting_pos;
    let mut current_direction = Direction::Up;

    while let Some(new_pos) = walk_direction(current_pos, &current_direction) {
        let next_tile = map.get(new_pos.0).and_then(|line| line.get(new_pos.1));
        match next_tile {
            None => break,
            Some('#') => current_direction = current_direction.turn_right(),
            Some('.' | '^') => {
                current_pos = new_pos;
                visited_tiles.insert(new_pos);
            }
            Some(tile) => {
                panic!("Unexpected next tile {tile}")
            }
        }
    }

    visited_tiles.len()
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let map: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.expect("valid line"))
        .map(|line| line.chars().collect())
        .collect();

    let starting_pos = map
        .iter()
        .enumerate()
        .find_map(|(line_idx, line)| {
            line.iter()
                .enumerate()
                .find(|(_, c)| **c == '^')
                .map(|(col_idx, _)| (line_idx, col_idx))
        })
        .expect("starting position");

    let path_tile_count = count_path_tiles(starting_pos, &map);

    println!("Path tile count is {path_tile_count}");

    Ok(())
}
