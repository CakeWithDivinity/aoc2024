use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn get_optimal_disk_layout(mut input: Vec<u32>) -> Vec<usize> {
    let mut optimal_ids = Vec::new();

    let input_is_even = input.len() % 2 == 0;
    let mut file_id = 0;

    let mut last_file_idx = if input_is_even {
        input.len() - 2
    } else {
        input.len() - 1
    };

    let mut idx = 0;

    while idx < input.len() && last_file_idx >= idx {
        if idx % 2 == 0 {
            let file_size = input[idx];

            if file_size == 0 {
                break;
            }

            optimal_ids.extend(vec![file_id; file_size as usize]);
            input[idx] = 0;
            idx += 1;
            file_id += 1;
        } else {
            let free_size = input[idx];
            if free_size == 0 {
                idx += 1;
                continue;
            }

            let last_file_size = input[last_file_idx];
            if last_file_size == 0 {
                last_file_idx -= 2;
                continue;
            }

            let last_file_id = last_file_idx / 2;
            optimal_ids.push(last_file_id);
            input[last_file_idx] -= 1;
            input[idx] -= 1;
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

    let input = lines
        .first()
        .unwrap()
        .chars()
        .map(|c| c.to_digit(10).expect("valid digit"))
        .collect::<Vec<u32>>();

    let optimal_disk_layout = get_optimal_disk_layout(input);

    let checksum: usize = optimal_disk_layout
        .iter()
        .enumerate()
        .map(|(idx, id)| idx * id)
        .sum();

    println!("Checksum is {checksum}");

    Ok(())
}
