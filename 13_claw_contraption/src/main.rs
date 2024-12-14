use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

use num::Integer;

fn parse_button_value(value: &str) -> isize {
    let (_, value) = value.split_once("+").expect("+ is present");

    value.parse().expect("valid isize")
}

fn parse_target_value(value: &str) -> isize {
    let (_, value) = value.split_once("=").expect("= is present");

    value.parse().expect("valid isize")
}

fn parse_target(line: &str) -> (isize, isize) {
    let (_, rest) = line.split_once(": ").expect("colon present");
    let (x, y) = rest.split_once(", ").expect("comma present");

    (parse_target_value(x), parse_target_value(y))
}

#[derive(Debug)]
struct Button {
    x: isize,
    y: isize,
    cost: isize,
}

impl From<&String> for Button {
    fn from(value: &String) -> Self {
        let (_, rest) = value
            .split_once("Button ")
            .expect("Button starts with Button");
        let (button_type, rest) = rest.split_once(": ").expect("colon is present");

        let cost = match button_type {
            "A" => 3,
            "B" => 1,
            c => panic!("Invalid button type {c}"),
        };

        let (x, y) = rest.split_once(", ").expect("comma present");
        let x = parse_button_value(x);
        let y = parse_button_value(y);

        Button { x, y, cost }
    }
}

fn get_determinant(a: isize, b: isize, c: isize, d: isize) -> isize {
    a * d - b * c
}

#[derive(Debug)]
struct Puzzle {
    button_a: Button,
    button_b: Button,
    target: (isize, isize),
}

impl Puzzle {
    fn get_min_cost(&self) -> Option<isize> {
        let gcd_x = self.button_a.x.gcd(&self.button_b.x);
        let gcd_y = self.button_a.y.gcd(&self.button_b.y);

        let has_x_solution = self.target.0 % gcd_x == 0;
        let has_y_solution = self.target.1 % gcd_y == 0;

        if !has_x_solution || !has_y_solution {
            return None;
        }

        let count_a = get_determinant(
            self.target.0,
            self.target.1,
            self.button_b.x,
            self.button_b.y,
        ) / get_determinant(
            self.button_a.x,
            self.button_a.y,
            self.button_b.x,
            self.button_b.y,
        );

        let count_b = get_determinant(
            self.button_a.x,
            self.button_a.y,
            self.target.0,
            self.target.1,
        ) / get_determinant(
            self.button_a.x,
            self.button_a.y,
            self.button_b.x,
            self.button_b.y,
        );

        let x_result = self.button_a.x * count_a + self.button_b.x * count_b;
        let y_result = self.button_a.y * count_a + self.button_b.y * count_b;

        if x_result != self.target.0 || y_result != self.target.1 {
            return None;
        }

        Some(count_a * self.button_a.cost + count_b * self.button_b.cost)
    }
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let lines = reader
        .lines()
        .map(|line| line.expect("valid line"))
        .collect::<Vec<_>>();

    let puzzles = lines
        .split(|line| line.is_empty())
        .map(|lines| {
            let mut lines = lines.iter();

            let button_a: Button = lines.next().expect("button a").into();
            let button_b: Button = lines.next().expect("button b").into();
            let target = parse_target(lines.next().expect("target"));

            let target = (target.0 + 10000000000000, target.1 + 10000000000000);

            Puzzle {
                button_a,
                button_b,
                target,
            }
        })
        .collect::<Vec<_>>();

    let costs: isize = puzzles
        .iter()
        .map(|puzzle| puzzle.get_min_cost().unwrap_or(0))
        .sum();

    println!("Min costs sum is {costs}");

    Ok(())
}
