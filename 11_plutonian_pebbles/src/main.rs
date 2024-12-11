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

fn blink(stone: usize, cache: &mut HashMap<usize, Vec<usize>>) -> Vec<usize> {
    if let Some(res) = cache.get(&stone) {
        return res.clone();
    }

    let result = if stone == 0 {
        vec![1]
    } else if let Some((left, right)) = split_if_even(stone) {
        vec![left, right]
    } else {
        vec![stone * 2024]
    };

    cache.insert(stone, result.clone());

    result
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .map(|line| line.expect("valid line"))
        .collect::<Vec<_>>();

    let mut stones = lines
        .first()
        .expect("first line")
        .split(' ')
        .map(|stone| stone.parse::<usize>().expect("valid number"))
        .collect::<Vec<_>>();

    let mut cache: HashMap<usize, Vec<usize>> = HashMap::new();

    for _ in 0..25 {
        stones = stones
            .iter()
            .flat_map(|stone| blink(*stone, &mut cache))
            .collect();
    }

    let stone_count = stones.len();
    println!("Stone count: {stone_count}");

    Ok(())
}
