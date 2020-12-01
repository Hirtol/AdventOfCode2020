use std::fs::File;
use advent_of_code_2020::{BasicOptions, read_puzzle_file};
use std::path::{Path, PathBuf};
use std::option::Option::Some;
use clap::Clap;

fn main() -> anyhow::Result<()>{
    let opts: BasicOptions = BasicOptions::parse();
    let path = PathBuf::from(opts.path_to_puzzles).join("day_1.txt");
    let puzzle = read_puzzle_file(path)?;
    let second_iterator = puzzle.clone().into_iter().map(|s| s.parse().unwrap()).collect::<Vec<u64>>();

    let mut result = (0, 0);

    for current_int in puzzle.into_iter() {
        let parsed: u64 = current_int.parse()?;
        let temp_result = second_iterator.iter().find(|to_add| **to_add + parsed == 2020).cloned();
        if let Some(to_add) = temp_result {
            result = (parsed, to_add);
            break;
        }
    }

    println!("Result for ints ({}, {}) is: {}", result.0, result.1, result.0 * result.1);

    Ok(())
}