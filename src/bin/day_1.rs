use advent_of_code_2020::{read_puzzle_file, BasicOptions};
use clap::Clap;

use std::option::Option::Some;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    let opts: BasicOptions = BasicOptions::parse();
    let path = PathBuf::from(opts.path_to_puzzles).join("day_1.txt");
    let puzzle = read_puzzle_file(path)?
        .into_iter()
        .flat_map(|s| s.parse())
        .collect::<Vec<u64>>();

    let first_result = find_two_numbers_for_2020(&puzzle, 2020).unwrap();
    let second_result = find_three_numbers_for_2020(&puzzle).unwrap();

    println!(
        "First Result for ints ({}, {}) is: {}",
        first_result.0,
        first_result.1,
        first_result.0 * first_result.1
    );
    println!(
        "Second Result for ints ({}, {}, {}) is: {}",
        second_result.0,
        second_result.1,
        second_result.2,
        second_result.0 * second_result.1 * second_result.2
    );

    Ok(())
}

fn find_two_numbers_for_2020(puzzle: &[u64], expected: u64) -> Option<(u64, u64)> {
    for first_int in puzzle {
        let temp_result = puzzle.iter().cloned().find(|to_add| to_add + first_int == expected);
        if let Some(second) = temp_result {
            return Some((*first_int, second));
        }
    }
    None
}

fn find_three_numbers_for_2020(puzzle: &[u64]) -> Option<(u64, u64, u64)> {
    for first_int in puzzle {
        if let Some((second, third)) = find_two_numbers_for_2020(puzzle, 2020 - first_int) {
            return Some((*first_int, second, third));
        }
    }
    None
}
