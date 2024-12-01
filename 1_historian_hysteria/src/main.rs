use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader, Error},
    vec,
};

fn main() -> Result<(), Error> {
    let input = File::open("input.txt")?;
    let reader = BufReader::new(input);
    let mut lines = reader.lines();

    let mut left_nums: Vec<usize> = vec![];
    let mut right_nums: Vec<usize> = vec![];

    while let Some(Ok(line)) = lines.next() {
        let mut nums = line.split_whitespace();

        left_nums.push(
            nums.next()
                .expect("left part")
                .parse()
                .expect("valid number"),
        );
        right_nums.push(
            nums.next()
                .expect("right part")
                .parse()
                .expect("valid number"),
        );
    }

    let right_num_occurence: HashMap<usize, usize> =
        right_nums.iter().fold(HashMap::new(), |mut acc, curr| {
            acc.insert(*curr, acc.get(curr).unwrap_or(&0) + 1);
            acc
        });

    let sum: usize = left_nums
        .iter()
        .map(|num| num * right_num_occurence.get(num).unwrap_or(&0))
        .sum();

    println!("Sum is {sum}");

    Ok(())
}
