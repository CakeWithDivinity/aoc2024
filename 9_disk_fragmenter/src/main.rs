use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

enum Block {
    File(u32, usize),
    Free(u32),
}

fn get_last_fitting_file_idx(free_size: u32, input: &[Block]) -> Option<usize> {
    let mut last_potential_file_idx = input.len() - 1;

    loop {
        let Block::File(file_size, _) = input[last_potential_file_idx] else {
            last_potential_file_idx -= 1;
            continue;
        };

        if file_size > 0 && free_size >= file_size {
            return Some(last_potential_file_idx);
        }

        if last_potential_file_idx < 1 {
            return None;
        }

        last_potential_file_idx -= 1;
    }
}

fn get_optimal_disk_layout(mut input: Vec<Block>) -> Vec<usize> {
    let mut optimal_ids = Vec::new();

    let mut idx = 0;

    // i have no idea what I am doing at this point, but it works
    while idx < input.len() {
        match input[idx] {
            Block::File(file_size, file_id) => {
                optimal_ids.extend(vec![file_id; file_size as usize]);
                input[idx] = Block::File(0, file_id);
                idx += 1;
            }
            Block::Free(free_size) => {
                if free_size == 0 {
                    idx += 1;
                    continue;
                }

                match get_last_fitting_file_idx(free_size, &input) {
                    Some(last_fitting_file_idx) => {
                        let Block::File(file_size, file_id) = input[last_fitting_file_idx] else {
                            panic!("get_last_fitting_file_idx returned a free block dafuq");
                        };

                        optimal_ids.extend(vec![file_id; file_size as usize]);

                        input[idx] = Block::Free(free_size - file_size);

                        input[last_fitting_file_idx] = Block::Free(file_size);
                    }
                    None => {
                        optimal_ids.extend(vec![0; free_size as usize]);
                        input[idx] = Block::Free(0);
                    }
                }
            }
        }
    }

    optimal_ids
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|line| line.expect("valid line"))
        .collect::<Vec<_>>();

    if lines.len() != 1 {
        panic!("Input is not one line");
    }

    let input: Vec<Block> = lines
        .first()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).expect("valid digit"))
        .enumerate()
        .map(|(idx, d)| {
            if idx % 2 == 0 {
                Block::File(d, idx / 2)
            } else {
                Block::Free(d)
            }
        })
        .collect::<Vec<_>>();

    let optimal_disk_layout = get_optimal_disk_layout(input);

    let checksum: usize = optimal_disk_layout
        .iter()
        .enumerate()
        .map(|(idx, id)| idx * id)
        .sum();

    println!("Checksum is {checksum}");

    Ok(())
}
