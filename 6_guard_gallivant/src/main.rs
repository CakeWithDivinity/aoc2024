use core::panic;
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader, Error},
};

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
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

fn get_path(
    starting_pos: (usize, usize),
    map: &[Vec<char>],
) -> Option<HashSet<(usize, usize, Direction)>> {
    let mut visited_tiles: HashSet<(usize, usize, Direction)> = HashSet::new();
    visited_tiles.insert((starting_pos.0, starting_pos.1, Direction::Up));

    let mut current_pos = starting_pos;
    let mut current_direction = Direction::Up;

    while let Some(new_pos) = walk_direction(current_pos, &current_direction) {
        let next_tile = map.get(new_pos.0).and_then(|line| line.get(new_pos.1));
        match next_tile {
            None => break,
            Some('#') => {
                current_direction = current_direction.turn_right();
            }
            Some('.' | '^') => {
                current_pos = new_pos;

                if visited_tiles.contains(&(new_pos.0, new_pos.1, current_direction.clone())) {
                    return None;
                }

                visited_tiles.insert((new_pos.0, new_pos.1, current_direction.clone()));
            }
            Some(tile) => {
                panic!("Unexpected next tile {tile}")
            }
        }
    }

    Some(visited_tiles)
}

fn get_obstacle_count(starting_pos: (usize, usize), map: &[Vec<char>]) -> usize {
    let visited_tiles = get_path(starting_pos, map).expect("normal path cannot have loop");

    let mut obstacle_map: HashSet<(usize, usize)> = HashSet::new();

    visited_tiles
        .iter()
        .filter_map(|(y, x, _)| {
            let mut new_map = map.to_owned();

            if *y >= new_map.len() || *x >= new_map[*y].len() {
                return None;
            }

            if new_map[*y][*x] == '#' {
                return None;
            }

            new_map[*y][*x] = '#';

            match get_path(starting_pos, &new_map) {
                None => Some((*y, *x)),
                Some(_) => None,
            }
        })
        .for_each(|obstacle_pos| {
            obstacle_map.insert(obstacle_pos);
        });

    obstacle_map.len()
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

    let obstacle_count = get_obstacle_count(starting_pos, &map);

    println!("Obstacle count is {obstacle_count}");

    Ok(())
}
