use advent_of_code_2020::{read_puzzle_file, BasicOptions, AdventOfCode};
use clap::Clap;

use std::collections::HashMap;
use std::option::Option::Some;
use std::path::PathBuf;

use itertools::Itertools;
use std::ops::RangeInclusive;

struct Day3;

impl AdventOfCode for Day3 {
    type PuzzleData = Vec<String>;

    fn part_one(puzzle: &Self::PuzzleData) {
        let (mut current_x, mut current_y) = (0, 0);
        let height = puzzle.len();
        let line_width = puzzle[0].len();
        let mut trees_seen = 0;

        while current_y < height {
            let position_char = puzzle[current_y].chars().nth(current_x % line_width).unwrap();
            if position_char == '#' {
                trees_seen += 1;
            }
            current_x += 3;
            current_y += 1;
        }

        println!("Part One, trees seen: {}", trees_seen);
    }

    fn part_two(puzzle: &Self::PuzzleData) {
        unimplemented!()
    }

    fn parse_file(puzzle: Vec<String>) -> Self::PuzzleData {
        puzzle
    }
}

fn main() -> anyhow::Result<()> {
    let opts: BasicOptions = BasicOptions::parse();
    Day3::start(PathBuf::from(opts.path_to_puzzles).join("day_3.txt"))
}
