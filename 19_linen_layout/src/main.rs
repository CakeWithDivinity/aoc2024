use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn is_design_possible(design: &str, towels: &[&str]) -> bool {
    let mut table: Vec<bool> = vec![false; design.len() + 1];
    table[0] = true;

    for i in 0..=design.len() {
        for towel in towels {
            let towel_len = towel.len();

            if i >= towel_len && design[i - towel_len..i] == **towel {
                table[i] = table[i] || table[i - towel_len]
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

    let possible_design_count = designs
        .iter()
        .filter(|design| is_design_possible(design, &towels))
        .count();

    println!("Possible design count: {possible_design_count}");

    Ok(())
}
