use advent_of_code_2020::{read_puzzle_file, BasicOptions};
use clap::Clap;
use std::fs::File;
use std::option::Option::Some;
use std::path::{Path, PathBuf};

fn main() -> anyhow::Result<()> {
    let opts: BasicOptions = BasicOptions::parse();
    let path = PathBuf::from(opts.path_to_puzzles).join("day_1.txt");
    let puzzle = read_puzzle_file(path)?
        .into_iter()
        .map(|s| s.parse().unwrap())
        .collect::<Vec<u64>>();

    let first_result = find_two_numbers_for_2020(&puzzle).unwrap();
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

fn find_two_numbers_for_2020(puzzle: &[u64]) -> Option<(u64, u64)> {
    for current_int in puzzle.iter() {
        let temp_result = puzzle.iter().find(|to_add| **to_add + *current_int == 2020).cloned();
        if let Some(to_add) = temp_result {
            return Some((*current_int, to_add));
        }
    }
    None
}

fn find_three_numbers_for_2020(puzzle: &[u64]) -> Option<(u64, u64, u64)> {
    for current_int in puzzle.iter() {
        for second_int in puzzle.iter() {
            let temp_result = puzzle
                .iter()
                .find(|to_add| **to_add + *current_int + *second_int == 2020)
                .cloned();
            if let Some(to_add) = temp_result {
                return Some((*current_int, *second_int, to_add));
            }
        }
    }
    None
}
