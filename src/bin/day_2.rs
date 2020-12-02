use advent_of_code_2020::{read_puzzle_file, BasicOptions};
use clap::Clap;

use std::option::Option::Some;
use std::path::PathBuf;
use std::collections::HashMap;
use rayon::iter::IntoParallelIterator;
use itertools::Itertools;
use std::ops::{Range, RangeInclusive};

fn main() -> anyhow::Result<()> {
    let opts: BasicOptions = BasicOptions::parse();
    let path = PathBuf::from(opts.path_to_puzzles).join("day_2.txt");
    let puzzle = read_puzzle_file(path)?;

    let map: HashMap<String, (RangeInclusive<usize>, char)> = puzzle.into_iter().map(|s| {
        let (policy, password) = s.split_at(s.find(':').unwrap() + 1);
        let (range, letter) = policy.split_at(policy.find(' ').unwrap());
        if let Some((lower_bound, higher_bound)) = range.split('-').flat_map(|i| i.parse()).collect_tuple() {
            return (password.trim().to_string(), (lower_bound..=higher_bound, letter.trim().chars().next().unwrap()))
        } else {
            panic!()
        }
    }).collect();

    part_one(&map);
    part_two(&map);

    Ok(())
}

fn part_one(puzzle: &HashMap<String, (RangeInclusive<usize>, char)>) {
    let mut correct_passwords = 0;
    for (k, v) in puzzle {
        let count = k.chars().filter(|to_check| *to_check == v.1).count();
        if v.0.contains(&count) {
            correct_passwords += 1;
        }
    }

    println!("Correct passwords part one: {}", correct_passwords);
}

fn part_two(puzzle: &HashMap<String, (RangeInclusive<usize>, char)>) {
    let correct_passwords = puzzle.iter().filter(|(password, v)| {
        let bottom_index = v.0.start().clone() - 1;
        let top_index = v.0.end().clone() - 1;

        let correct_amount = password.char_indices()
            .filter(|c| c.0 == bottom_index || c.0 == top_index)
            .filter(|c| c.1 == v.1)
            .count();

        correct_amount == 1
    }).count();

    println!("Correct passwords part two: {}", correct_passwords);
}
