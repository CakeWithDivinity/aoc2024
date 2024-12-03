use std::{
    fs::File,
    io::{BufRead, BufReader, Error},
    iter::Peekable,
};

macro_rules! assert_next_items {
    ($char_iter:expr, $($val:expr),*) => {
        $(
            $char_iter.next_if(|n| **n == $val)?;
        )*
    };
}

fn parse_digits<'a, I>(iter: &mut Peekable<I>) -> Option<usize>
where
    I: Iterator<Item = &'a char>,
{
    let mut digits: Vec<char> = vec![];
    while let Some(n) = iter.peek() {
        if n.is_ascii_digit() {
            digits.push(**n);
            iter.next();
        } else {
            break;
        }
    }

    if digits.is_empty() {
        None
    } else {
        let digit_str: String = digits.iter().collect();
        Some(digit_str.parse().expect("valid number"))
    }
}

fn parse_potential_mul<'a, I>(char_iter: &mut Peekable<I>) -> Option<usize>
where
    I: Iterator<Item = &'a char>,
{
    assert_next_items!(char_iter, 'u', 'l', '(');

    let left = parse_digits(char_iter)?;

    assert_next_items!(char_iter, ',');

    let right = parse_digits(char_iter)?;

    assert_next_items!(char_iter, ')');

    Some(left * right)
}

fn parse_potential_instr<'a, I>(char_iter: &mut Peekable<I>) -> Option<bool>
where
    I: Iterator<Item = &'a char>,
{
    assert_next_items!(char_iter, 'o');

    match char_iter.peek() {
        Some('(') => {
            char_iter.next();
            assert_next_items!(char_iter, ')');
            Some(true)
        }
        Some('n') => {
            char_iter.next();
            assert_next_items!(char_iter, '\'', 't', '(', ')');
            Some(false)
        }
        _ => None,
    }
}

fn accumulate_muls(chars: &[char]) -> usize {
    let mut char_iter = chars.iter().peekable();

    let mut sum: usize = 0;
    let mut last_instr: bool = true;
    // not using regex cause we ballin
    while let Some(c) = char_iter.next() {
        match *c {
            'm' => {
                if let Some(result) = parse_potential_mul(&mut char_iter) {
                    if last_instr {
                        sum += result;
                    }
                }
            }
            'd' => {
                if let Some(instr) = parse_potential_instr(&mut char_iter) {
                    last_instr = instr;
                }
            }
            _ => {
                continue;
            }
        }
    }

    sum
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);

    let chars: Vec<char> = reader
        .lines()
        .flat_map(|line| line.expect("valid line").chars().collect::<Vec<char>>())
        .collect();

    let sum = accumulate_muls(&chars);
    println!("Sum is {sum}");

    Ok(())
}
