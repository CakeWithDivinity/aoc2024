use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

const MAX_X: isize = 100;
const MAX_Y: isize = 102;
const WALK_AMOUNT: usize = 100;

fn parse_xy(value: &str) -> (isize, isize) {
    let (_, xy) = value.split_once('=').expect("xy seperated by =");
    let (x, y) = xy.split_once(',').expect("xy values seperated by ,");
    let x = x.parse().expect("valid isize");
    let y = y.parse().expect("valid isize");

    (x, y)
}

fn wrap_in_bounding_box(value: isize, max: isize) -> isize {
    if value < 0 {
        max + value + 1
    } else if value > max {
        value - max - 1
    } else {
        value
    }
}

struct Robot {
    starting_pos: (isize, isize),
    velocity: (isize, isize),
}

impl Robot {
    fn simulate(&self) -> (isize, isize) {
        let mut pos = self.starting_pos;

        for _ in 0..WALK_AMOUNT {
            pos = self.walk(pos);
        }

        pos
    }

    fn walk(&self, pos: (isize, isize)) -> (isize, isize) {
        let new_pos = (pos.0 + self.velocity.0, pos.1 + self.velocity.1);

        (
            wrap_in_bounding_box(new_pos.0, MAX_X),
            wrap_in_bounding_box(new_pos.1, MAX_Y),
        )
    }
}

impl From<String> for Robot {
    fn from(value: String) -> Self {
        let (pos, v) = value.split_once(' ').expect("seperated by space");

        let pos = parse_xy(pos);
        let v = parse_xy(v);

        Robot {
            starting_pos: pos,
            velocity: v,
        }
    }
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let robots = reader
        .lines()
        .map(|line| line.expect("valid line"))
        .map(|line| line.into())
        .collect::<Vec<Robot>>();

    let x_middle = MAX_X / 2;
    let y_middle = MAX_Y / 2;

    let mut quadrants = [0; 4];

    robots.iter().map(|robot| robot.simulate()).for_each(|pos| {
        if pos.0 == x_middle || pos.1 == y_middle {
            return;
        }

        if pos.0 < x_middle {
            if pos.1 < y_middle {
                quadrants[0] += 1;
            } else {
                quadrants[1] += 1;
            }
        } else if pos.1 < y_middle {
            quadrants[2] += 1;
        } else {
            quadrants[3] += 1;
        }
    });

    let result: usize = quadrants.iter().product();

    println!("Result is {result}");

    Ok(())
}
