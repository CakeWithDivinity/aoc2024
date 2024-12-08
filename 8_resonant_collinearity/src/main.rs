use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Error},
};

use itertools::Itertools;

type Position = (isize, isize);
type AntennaMap = HashMap<char, Vec<Position>>;

fn get_antenna_map(map: &[Vec<char>]) -> AntennaMap {
    let mut antenna_map: AntennaMap = AntennaMap::new();

    for (line_idx, line) in map.iter().enumerate() {
        for (col_idx, col) in line.iter().enumerate() {
            if *col != '.' {
                let list = antenna_map.entry(*col).or_default();
                list.push((line_idx as isize, col_idx as isize));
            }
        }
    }

    antenna_map
}

fn get_antinode_positions(a: &Position, b: &Position) -> (Position, Position) {
    let y_diff = a.0 - b.0;
    let x_diff = a.1 - b.1;

    let pos1 = (a.0 - 2 * y_diff, a.1 - 2 * x_diff);
    let pos2 = (b.0 + 2 * y_diff, b.1 + 2 * x_diff);

    (pos1, pos2)
}

fn is_valid_antinode_pos(pos: &Position, width: isize, height: isize) -> bool {
    pos.0 >= 0 && pos.0 < width && pos.1 >= 0 && pos.1 < height
}

fn get_antinode_count(map: &[Vec<char>]) -> usize {
    let antenna_map = get_antenna_map(map);

    let height = map.len() as isize;
    let width = map[0].len() as isize;

    let mut antinodes: HashSet<Position> = HashSet::new();

    let antenna_combinations: Vec<(&Position, &Position)> = antenna_map
        .iter()
        .flat_map(|(_, antennas)| antennas.iter().combinations(2))
        .map(|antenna_combination| (antenna_combination[0], antenna_combination[1]))
        .collect();

    for (a, b) in antenna_combinations {
        let antinode_positions = get_antinode_positions(a, b);

        if is_valid_antinode_pos(&antinode_positions.0, width, height) {
            antinodes.insert(antinode_positions.0);
        }

        if is_valid_antinode_pos(&antinode_positions.1, width, height) {
            antinodes.insert(antinode_positions.1);
        }
    }

    antinodes.len()
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let lines: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.expect("valid line"))
        .map(|line| line.chars().collect())
        .collect();

    let count = get_antinode_count(&lines);

    println!("Antinode count is {count}");

    Ok(())
}
