use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Error},
};

const DIRECTIONS: &[(isize, isize)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

fn get_reachable_end_positions(
    map: &[Vec<u32>],
    cache: &mut HashMap<(usize, usize), Vec<(usize, usize)>>,
    position: (usize, usize),
) -> Vec<(usize, usize)> {
    if let Some(end_positions) = cache.get(&position) {
        return end_positions.clone();
    }

    let curr_height = map[position.0][position.1];

    if curr_height == 9 {
        return vec![position];
    }

    let reachable_end_positions = DIRECTIONS
        .iter()
        .filter_map(|direction| {
            let new_pos = (
                position.0.checked_add_signed(direction.0)?,
                position.1.checked_add_signed(direction.1)?,
            );

            let new_height = map.get(new_pos.0)?.get(new_pos.1)?;

            if *new_height != curr_height + 1 {
                return None;
            }

            Some(get_reachable_end_positions(map, cache, new_pos))
        })
        .flatten()
        .collect::<HashSet<(usize, usize)>>();

    let unique_end_positions: Vec<(usize, usize)> = reachable_end_positions.into_iter().collect();

    cache.insert(position, unique_end_positions.clone());

    unique_end_positions
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let map: Vec<Vec<u32>> = reader
        .lines()
        .map(|line| line.expect("valid line"))
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("valid digit"))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let mut cache: HashMap<(usize, usize), Vec<(usize, usize)>> = HashMap::new();

    let score_sum = map
        .iter()
        .enumerate()
        .map(|(line_idx, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, height)| **height == 0)
                .map(|(col_idx, _)| {
                    get_reachable_end_positions(&map, &mut cache, (line_idx, col_idx)).len()
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("Score sum is {score_sum}");

    Ok(())
}
