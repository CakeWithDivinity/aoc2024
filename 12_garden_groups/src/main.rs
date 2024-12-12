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

fn get_new_char(pos: (usize, usize), dir: (isize, isize), grid: &[Vec<char>]) -> Option<char> {
    let new_pos = get_new_pos(pos, dir)?;

    grid.get(new_pos.0)
        .and_then(|line| line.get(new_pos.1))
        .copied()
}

fn get_corner_count(grid: &[Vec<char>], pos: (usize, usize), c: char) -> usize {
    let (y, x) = pos;

    let mut corners = 0;

    for dy in [-1, 1] {
        for dx in [-1, 1] {
            let char_y = get_new_char((y, x), (dy, 0), grid);
            let char_x = get_new_char((y, x), (0, dx), grid);
            let char_yx = get_new_char((y, x), (dy, dx), grid);

            let is_same_y = char_y.map(|char_y| char_y == c).unwrap_or(false);
            let is_same_x = char_x.map(|char_x| char_x == c).unwrap_or(false);
            let is_same_yx = char_yx.map(|char_yx| char_yx == c).unwrap_or(false);

            if is_same_x == is_same_y && !(is_same_x && is_same_yx) {
                corners += 1;
            }
        }
    }

    corners
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
            let mut _perimeter = 0;
            let mut sides = 0;

            visited_tiles.insert((line_idx, col_idx));

            let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
            queue.push_back((line_idx, col_idx));

            while let Some((y, x)) = queue.pop_front() {
                area += 1;

                sides += get_corner_count(grid, (y, x), *c);

                for direction in DIRECTIONS {
                    let Some(new_pos) = get_new_pos((y, x), *direction) else {
                        _perimeter += 1;
                        continue;
                    };

                    let new_char = grid.get(new_pos.0).and_then(|line| line.get(new_pos.1));

                    if new_char.is_none_or(|new_char| new_char != c) {
                        _perimeter += 1;
                    } else if !visited_tiles.contains(&new_pos) {
                        visited_tiles.insert(new_pos);
                        queue.push_back(new_pos);
                    }
                }
            }

            cost += area * sides;
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
