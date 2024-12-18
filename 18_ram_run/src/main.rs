use core::panic;
use std::{
    collections::{HashMap, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Error},
};

const GRID_WIDTH: usize = 71;
const GRID_HEIGHT: usize = 71;

const DIRECTIONS: &[(isize, isize); 4] = &[(-1, 0), (1, 0), (0, -1), (0, 1)];

fn get_shortest_path(grid: &[[bool; GRID_WIDTH]; GRID_HEIGHT]) -> Option<usize> {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut tile_dist: HashMap<(usize, usize), usize> = HashMap::new();

    queue.push_back((0, 0));
    tile_dist.insert((0, 0), 0);

    while let Some(pos) = queue.pop_front() {
        if pos.0 == GRID_HEIGHT - 1 && pos.1 == GRID_WIDTH - 1 {
            return Some(*tile_dist.get(&pos).expect("goal has cost"));
        }

        for direction in DIRECTIONS {
            let Some(new_y) = pos.0.checked_add_signed(direction.0) else {
                continue;
            };
            let Some(new_x) = pos.1.checked_add_signed(direction.1) else {
                continue;
            };

            let Some(is_corrupted) = grid.get(new_y).and_then(|line| line.get(new_x)) else {
                continue;
            };

            if *is_corrupted || tile_dist.contains_key(&(new_y, new_x)) {
                continue;
            }

            let curr_cost = tile_dist.get(&pos).expect("was visited before");
            tile_dist.insert((new_y, new_x), curr_cost + 1);
            queue.push_back((new_y, new_x));
        }
    }

    None
}

fn main() -> Result<(), Error> {
    let mut grid = [[false; GRID_WIDTH]; GRID_HEIGHT];

    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut lines = reader.lines();

    while let Some(Ok(line)) = lines.next() {
        let Some((x, y)) = line.split_once(",") else {
            panic!("Expected line to follow x,y. Got {line}")
        };

        grid[y.parse::<usize>().expect("valid usize")][x.parse::<usize>().expect("valid usize")] =
            true;

        if let Some(shortest_path) = get_shortest_path(&grid) {
            println!("Shortest path is {shortest_path}");
        } else {
            println!("Grid not traversible possible after: {line}");
            break;
        };
    }

    Ok(())
}
