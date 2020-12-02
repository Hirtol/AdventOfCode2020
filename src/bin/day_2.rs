use advent_of_code_2020::{read_puzzle_file, BasicOptions, AdventOfCode};
use clap::Clap;

use std::collections::HashMap;
use std::option::Option::Some;
use std::path::PathBuf;

use itertools::Itertools;
use std::ops::RangeInclusive;

struct Day2;

impl AdventOfCode for Day2 {
    type PuzzleData = HashMap<String, (RangeInclusive<usize>, char)>;

    fn part_one(puzzle: &Self::PuzzleData) {
        let correct_passwords = puzzle
            .iter()
            .map(|(k, v)| (k.chars().filter(|to_check| *to_check == v.1).count(), v))
            .filter(|(k, v)| v.0.contains(k))
            .count();

        println!("Correct passwords part one: {}", correct_passwords);
    }

    fn part_two(puzzle: &Self::PuzzleData) {
        let correct_passwords = puzzle
            .iter()
            .filter(|(password, v)| {
                let bottom_index = v.0.start().clone() - 1;
                let top_index = v.0.end().clone() - 1;

                let correct_amount = password
                    .char_indices()
                    .filter(|c| c.0 == bottom_index || c.0 == top_index)
                    .filter(|c| c.1 == v.1)
                    .count();

                correct_amount == 1
            })
            .count();

        println!("Correct passwords part two: {}", correct_passwords);
    }

    fn parse_file(puzzle: Vec<String>) -> Self::PuzzleData {
        puzzle.into_iter()
            .map(|s| {
                let (policy, password) = s.split_at(s.find(':').unwrap() + 1);
                let (range, letter) = policy.split_at(policy.find(' ').unwrap());
                let (lower_bound, higher_bound) = range.split('-').flat_map(|i| i.parse()).collect_tuple().unwrap();
                (
                    password.trim().to_string(),
                    (lower_bound..=higher_bound, letter.trim().chars().next().unwrap()),
                )
            })
            .collect()
    }
}

fn main() -> anyhow::Result<()> {
    let opts: BasicOptions = BasicOptions::parse();
    Day2::start(PathBuf::from(opts.path_to_puzzles).join("day_2.txt"))
}
