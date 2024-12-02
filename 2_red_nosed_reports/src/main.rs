use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn all_have_same_sign(entries: &Vec<isize>) -> bool {
    if entries[0].is_positive() {
        entries.iter().all(|entry| entry.is_positive())
    } else {
        entries.iter().all(|entry| entry.is_negative())
    }
}

fn is_valid_record(record: &Vec<isize>) -> bool {
    let diffs: Vec<isize> = record
        .windows(2)
        .map(|window| {
            let a = window.get(0).expect("left");
            let b = window.get(1).expect("right");

            a - b
        })
        .collect();

    diffs.iter().all(|diff| diff.abs() > 0 && diff.abs() < 4) && all_have_same_sign(&diffs)
}

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);
    let mut lines = reader.lines();

    let mut records: Vec<Vec<isize>> = Vec::new();

    while let Some(Ok(line)) = lines.next() {
        records.push(
            line.split_whitespace()
                .map(|entry| entry.parse().expect("valid usize"))
                .collect(),
        );
    }

    let valid_record_count = records
        .iter()
        .filter(|record| {
            is_valid_record(record)
                || (0..record.len()).any(|remove_idx| {
                    let mut new_record = (*record).clone();
                    new_record.remove(remove_idx);
                    is_valid_record(&new_record)
                })
        })
        .count();

    println!("Valid record count: {valid_record_count}");

    Ok(())
}
