use core::panic;
use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

fn parse_button_value(value: &str) -> usize {
    let (_, value) = value.split_once("+").expect("+ is present");

    value.parse().expect("valid usize")
}

fn parse_target_value(value: &str) -> usize {
    let (_, value) = value.split_once("=").expect("= is present");

    value.parse().expect("valid usize")
}

fn parse_target(line: &str) -> (usize, usize) {
    let (_, rest) = line.split_once(": ").expect("colon present");
    let (x, y) = rest.split_once(", ").expect("comma present");

    (parse_target_value(x), parse_target_value(y))
}

#[derive(Debug)]
struct Button {
    x: usize,
    y: usize,
    cost: usize,
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

fn get_max_button_presses(button: &Button, target: &(usize, usize)) -> usize {
    let max_x_presses = target.0 / button.x;
    let max_y_presses = target.1 / button.y;

    max_x_presses.min(max_y_presses)
}

#[derive(Debug)]
struct Puzzle {
    button_a: Button,
    button_b: Button,
    target: (usize, usize),
}

impl Puzzle {
    fn get_min_cost(&self) -> Option<usize> {
        // let gcd_x = self.button_a.x.gcd(&self.button_b.x);
        // let gcd_y = self.button_a.y.gcd(&self.button_b.y);

        // let has_x_solution = self.target.0 % gcd_x == 0;
        // let has_y_solution = self.target.1 % gcd_y == 0;

        // if !has_x_solution || !has_y_solution {
        //     return None;
        // }

        let mut costs: Vec<usize> = Vec::new();

        for a in 0..=100 {
            let button_a_x = self.button_a.x * a;
            let button_a_y = self.button_a.y * a;

            if button_a_x > self.target.0 || button_a_y > self.target.1 {
                break;
            }

            for b in 0..=100 {
                let button_b_x = self.button_b.x * b;
                let button_b_y = self.button_b.y * b;

                let sum_x = button_a_x + button_b_x;
                let sum_y = button_a_y + button_b_y;

                if sum_x > self.target.0 || sum_y > self.target.1 {
                    break;
                }

                if sum_x == self.target.0 && sum_y == self.target.1 {
                    costs.push(a * self.button_a.cost + b * self.button_b.cost);
                }
            }
        }

        costs.into_iter().min()
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

            Puzzle {
                button_a,
                button_b,
                target,
            }
        })
        .collect::<Vec<_>>();

    let costs: usize = puzzles
        .iter()
        .map(|puzzle| puzzle.get_min_cost().unwrap_or(0))
        .sum();

    println!("Min costs sum is {costs}");

    Ok(())
}
