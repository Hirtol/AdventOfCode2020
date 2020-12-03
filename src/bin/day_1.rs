use advent_of_code_2020::{read_puzzle_file, AdventOfCode, BasicOptions};
use clap::Clap;

use std::option::Option::Some;
use std::path::PathBuf;

struct Day1;
impl AdventOfCode for Day1 {
    type PuzzleData = Vec<u64>;

    fn part_one(puzzle: &Self::PuzzleData) {
        let first_result = find_two_numbers_for_2020(puzzle, 2020).unwrap();
        println!(
            "First Result for ints ({}, {}) is: {}",
            first_result.0,
            first_result.1,
            first_result.0 * first_result.1
        );
    }

    fn part_two(puzzle: &Self::PuzzleData) {
        for first_int in puzzle {
            if let Some((second, third)) = find_two_numbers_for_2020(puzzle, 2020 - first_int) {
                println!(
                    "Second Result for ints ({}, {}, {}) is: {}",
                    first_int,
                    second,
                    third,
                    first_int * second * third
                );
                break;
            }
        }
    }

    fn parse_file(puzzle: Vec<String>) -> Self::PuzzleData {
        puzzle.into_iter().flat_map(|s| s.parse()).collect::<Vec<u64>>()
    }
}

fn find_two_numbers_for_2020(puzzle: &[u64], expected: u64) -> Option<(u64, u64)> {
    for first_int in puzzle {
        let expect = expected - first_int;
        if puzzle.contains(&expect) {
            return Some((*first_int, expect));
        }
    }
    None
}

fn main() -> anyhow::Result<()> {
    let opts: BasicOptions = BasicOptions::parse();
    Day1::start(PathBuf::from(opts.path_to_puzzles).join("day_1.txt"))
}
