use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn get_towel_combination_count(design: &str, towels: &[&str]) -> usize {
    let mut table = vec![0; design.len() + 1];
    table[0] = 1;

    for i in 0..=design.len() {
        for towel in towels {
            let towel_len = towel.len();

            if i >= towel_len && design[i - towel_len..i] == **towel {
                table[i] += table[i - towel_len];
            }
        }
    }

    table[design.len()]
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .map(|line| line.expect("valid line"))
        .collect::<Vec<_>>();

    let mut split = lines.split(|line| line.is_empty());
    let towels = split.next().expect("towels to be present");
    let designs = split.next().expect("designs to be present");

    let towels = towels
        .iter()
        .flat_map(|line| line.split(", "))
        .collect::<Vec<_>>();

    let possible_design_count: usize = designs
        .iter()
        .map(|design| get_towel_combination_count(design, &towels))
        .sum();

    println!("Possible design count: {possible_design_count}");

    Ok(())
}
