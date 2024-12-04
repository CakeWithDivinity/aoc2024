use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
};

enum Direction {
    Left,
    Right,
    Up,
    Down,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Direction {
    fn to_idx_diff(&self) -> (isize, isize) {
        match self {
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::UpLeft => (-1, -1),
            Direction::UpRight => (-1, 1),
            Direction::DownLeft => (1, -1),
            Direction::DownRight => (1, 1),
        }
    }

    fn get_opposite(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::UpLeft => Direction::DownRight,
            Direction::UpRight => Direction::DownLeft,
            Direction::DownLeft => Direction::UpRight,
            Direction::DownRight => Direction::UpLeft,
        }
    }
}

static ALL_DIRS: &[Direction] = &[
    // Direction::Right,
    // Direction::Left,
    // Direction::Up,
    // Direction::Down,
    Direction::UpLeft,
    Direction::UpRight,
    Direction::DownLeft,
    Direction::DownRight,
];

fn apply_direction_to_position(
    direction: &Direction,
    position: (usize, usize),
) -> Option<(usize, usize)> {
    let idx_diff = direction.to_idx_diff();

    let new_y = position.0.checked_add_signed(idx_diff.0)?;
    let new_x = position.1.checked_add_signed(idx_diff.1)?;

    Some((new_y, new_x))
}

fn get_char_in_dir(
    vec: &[Vec<char>],
    direction: &Direction,
    position: (usize, usize),
) -> Option<char> {
    let pos = apply_direction_to_position(direction, position)?;

    let char = vec.get(pos.0).map(|line| line.get(pos.1))??;

    Some(*char)
}

fn is_valid_xmas_at_pos(vec: &[Vec<char>], line_idx: usize, char_idx: usize) -> bool {
    let pos = (line_idx, char_idx);
    if vec[line_idx][char_idx] != 'A' {
        return false;
    }

    let mut mas_count = 0;

    for dir in ALL_DIRS {
        if get_char_in_dir(vec, dir, pos).map_or(true, |c| c != 'M') {
            continue;
        }

        if get_char_in_dir(vec, &dir.get_opposite(), pos).map_or(true, |c| c != 'S') {
            continue;
        }

        mas_count += 1;
    }

    mas_count == 2
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let chars: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.expect("valid line").chars().collect())
        .collect();

    let mut sum = 0;
    for (line_idx, line) in chars.iter().enumerate() {
        for (char_idx, _) in line.iter().enumerate() {
            if is_valid_xmas_at_pos(&chars, line_idx, char_idx) {
                sum += 1;
            }
        }
    }

    print!("Sum is {sum}");

    Ok(())
}
