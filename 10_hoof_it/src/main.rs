use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error},
};

const DIRECTIONS: &[(isize, isize)] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

fn get_rating(
    map: &[Vec<u32>],
    cache: &mut HashMap<(usize, usize), usize>,
    position: (usize, usize),
) -> usize {
    if let Some(rating) = cache.get(&position) {
        return *rating;
    }

    let curr_height = map[position.0][position.1];

    if curr_height == 9 {
        return 1;
    }

    let rating: usize = DIRECTIONS
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

            Some(get_rating(map, cache, new_pos))
        })
        .sum();

    cache.insert(position, rating);

    rating
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

    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

    let score_sum = map
        .iter()
        .enumerate()
        .map(|(line_idx, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, height)| **height == 0)
                .map(|(col_idx, _)| get_rating(&map, &mut cache, (line_idx, col_idx)))
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("Score sum is {score_sum}");

    Ok(())
}
