use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn is_valid_equation(target: usize, current: usize, numbers: &[usize]) -> bool {
    if current > target {
        return false;
    }

    let Some((next_num, numbers_left)) = numbers.split_first() else {
        return current == target;
    };

    is_valid_equation(target, current + next_num, numbers_left)
        || is_valid_equation(target, current * next_num, numbers_left)
}

fn sum_valid_equations(equations: &[(usize, Vec<usize>)]) -> usize {
    equations
        .iter()
        .filter(|(target, numbers)| {
            let (first_num, numbers_left) = numbers.split_first().expect("more than one number");
            is_valid_equation(*target, *first_num, numbers_left)
        })
        .map(|(target, _)| target)
        .sum()
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let equations: Vec<(usize, Vec<usize>)> = reader
        .lines()
        .map(|line| line.expect("valid line"))
        .map(|line| {
            let (result, numbers) = line.split_once(": ").expect("result and numbers");

            let result = result.parse::<usize>().expect("valid usize");

            let numbers: Vec<usize> = numbers
                .split(" ")
                .map(|entry| entry.parse::<usize>().expect("valid usize"))
                .collect();

            (result, numbers)
        })
        .collect();

    let sum = sum_valid_equations(&equations);

    println!("Sum is {sum}");

    Ok(())
}
