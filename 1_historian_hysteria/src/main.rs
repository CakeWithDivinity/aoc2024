use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    iter::zip,
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

    left_nums.sort();
    right_nums.sort();

    let sum: usize = zip(left_nums, right_nums)
        .map(|(left, right)| left.abs_diff(right))
        .sum();

    println!("Sum is {sum}");

    Ok(())
}
