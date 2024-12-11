use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn split_if_even(num: usize) -> Option<(usize, usize)> {
    let num_str = num.to_string();

    if num_str.len() % 2 != 0 {
        return None;
    }

    let left = &num_str[0..num_str.len() / 2]
        .parse::<usize>()
        .expect("valid num");
    let right = &num_str[num_str.len() / 2..]
        .parse::<usize>()
        .expect("valid num");

    Some((*left, *right))
}

fn blink(stone: usize, blinks: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    let Some(blinks) = blinks.checked_sub(1) else {
        return 1;
    };

    if let Some(res) = cache.get(&(stone, blinks)) {
        return *res;
    }

    let result = if stone == 0 {
        blink(1, blinks, cache)
    } else if let Some((left, right)) = split_if_even(stone) {
        blink(left, blinks, cache) + blink(right, blinks, cache)
    } else {
        blink(stone * 2024, blinks, cache)
    };

    cache.insert((stone, blinks), result);

    result
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .map(|line| line.expect("valid line"))
        .collect::<Vec<_>>();

    let stones = lines
        .first()
        .expect("first line")
        .split(' ')
        .map(|stone| stone.parse::<usize>().expect("valid number"))
        .collect::<Vec<_>>();

    let mut cache: HashMap<(usize, usize), usize> = HashMap::new();

    let stone_count: usize = stones
        .iter()
        .map(|stone| blink(*stone, 75, &mut cache))
        .sum();

    println!("Stone count: {stone_count}");

    Ok(())
}
