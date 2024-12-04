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
}

static ALL_DIRS: &[Direction] = &[
    Direction::Right,
    Direction::Left,
    Direction::Up,
    Direction::Down,
    Direction::UpLeft,
    Direction::UpRight,
    Direction::DownLeft,
    Direction::DownRight,
];

static LETTERS: &[char] = &['X', 'M', 'A', 'S'];

fn apply_direction_to_position(
    direction: &Direction,
    position: (usize, usize),
) -> Option<(usize, usize)> {
    let idx_diff = direction.to_idx_diff();

    let new_y = position.0.checked_add_signed(idx_diff.0)?;
    let new_x = position.1.checked_add_signed(idx_diff.1)?;

    Some((new_y, new_x))
}

fn valid_xmas_count_at_pos(vec: &[Vec<char>], line_idx: usize, char_idx: usize) -> usize {
    if vec[line_idx][char_idx] != LETTERS[0] {
        return 0;
    }

    let mut valid_count = 0;

    'dir_loop: for dir in ALL_DIRS {
        let mut curr_pos = (line_idx, char_idx);

        for letter in &LETTERS[1..] {
            let Some(next_pos) = apply_direction_to_position(dir, curr_pos) else {
                continue 'dir_loop;
            };

            curr_pos = next_pos;

            let Some(Some(c)) = vec.get(curr_pos.0).map(|line| line.get(curr_pos.1)) else {
                continue 'dir_loop;
            };

            if c != letter {
                continue 'dir_loop;
            }
        }

        valid_count += 1;
    }

    valid_count
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
            sum += valid_xmas_count_at_pos(&chars, line_idx, char_idx);
        }
    }

    print!("Sum is {sum}");

    Ok(())
}
