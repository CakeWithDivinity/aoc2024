use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{BufRead, BufReader, Error},
};

/// specifies which numbers must come after the key
type Rules = HashMap<usize, HashSet<usize>>;

fn get_rules(rules: &[String]) -> Rules {
    let mut map: Rules = HashMap::new();

    for rule in rules {
        let (left, right) = rule.split_once('|').expect("seperated by |");

        let left_num: usize = left.parse().expect("valid number");
        let right_num: usize = right.parse().expect("valid number");

        let set = map.entry(left_num).or_default();

        set.insert(right_num);
    }

    map
}

fn is_valid_update(update: &[usize], rules: &Rules) -> bool {
    for (idx, entry) in update.iter().enumerate() {
        if update[..idx].iter().any(|num| {
            rules
                .get(entry)
                .is_some_and(|nums_after| nums_after.contains(num))
        }) {
            return false;
        }
    }

    true
}

fn main() -> Result<(), Error> {
    let file = File::open("input.txt")?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().map_while(|line| line.ok()).collect();

    let split_idx = lines
        .iter()
        .position(|line| line.is_empty())
        .expect("one empty line");

    let (rules, updates) = lines.split_at_mut(split_idx);

    let mut updates = updates.to_vec();
    updates.remove(0);

    let updates: Vec<Vec<usize>> = updates
        .iter()
        .map(|update| {
            update
                .split(',')
                .map(|entry| entry.parse().expect("valid number"))
                .collect()
        })
        .collect();

    let rules_map = get_rules(rules);

    let sum: usize = updates
        .iter()
        .filter(|update| is_valid_update(update, &rules_map))
        .map(|update| update[update.len() / 2])
        .sum();

    println!("Sum is {sum}");

    Ok(())
}
