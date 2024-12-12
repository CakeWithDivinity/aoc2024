use std::{
    collections::{HashSet, VecDeque},
    fs::File,
    io::{BufRead, BufReader, Error},
};

const DIRECTIONS: &[(isize, isize)] = &[(-1, 0), (1, 0), (0, 1), (0, -1)];

fn get_new_pos(pos: (usize, usize), dir: (isize, isize)) -> Option<(usize, usize)> {
    let new_y = pos.0.checked_add_signed(dir.0)?;
    let new_x = pos.1.checked_add_signed(dir.1)?;

    Some((new_y, new_x))
}

fn get_fence_cost(grid: &[Vec<char>]) -> usize {
    let mut cost = 0;

    let mut visited_tiles: HashSet<(usize, usize)> = HashSet::new();

    for (line_idx, line) in grid.iter().enumerate() {
        for (col_idx, c) in line.iter().enumerate() {
            if visited_tiles.contains(&(line_idx, col_idx)) {
                continue;
            }

            let mut area = 0;
            let mut perimeter = 0;
            visited_tiles.insert((line_idx, col_idx));

            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
            queue.push_back((line_idx, col_idx));

            while let Some((y, x)) = queue.pop_front() {
                area += 1;

                for direction in DIRECTIONS {
                    let Some(new_pos) = get_new_pos((y, x), *direction) else {
                        perimeter += 1;
                        continue;
                    };

                    let new_char = grid.get(new_pos.0).and_then(|line| line.get(new_pos.1));

                    if new_char.is_none_or(|new_char| new_char != c) {
                        perimeter += 1;
                    } else if !visited_tiles.contains(&new_pos) {
                        visited_tiles.insert(new_pos);
                        queue.push_back(new_pos);
                    }
                }
            }

            dbg!(c, area, perimeter);

            cost += area * perimeter;
        }
    }

    cost
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let grid = reader
        .lines()
        .map(|line| line.expect("valid line"))
        .map(|line| line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let fence_cost = get_fence_cost(&grid);

    println!("Fence cost is {fence_cost}");

    Ok(())
}
