use advent_of_code_2020::{read_puzzle_file, AdventOfCode, BasicOptions};
use clap::Clap;

use std::collections::HashMap;
use std::option::Option::Some;
use std::path::PathBuf;

use itertools::Itertools;
use std::ops::RangeInclusive;

struct Day3;

fn find_slope_trees(puzzle: &Vec<String>, x_increment: usize, y_increment: usize) -> usize {
    let line_width = puzzle[0].len();

    puzzle.iter()
        .step_by(y_increment)
        .enumerate()
        .flat_map(|(line_y, line)| line.chars().nth((line_y * x_increment) % line_width))
        .filter(|&c| c == '#')
        .count()
}

impl AdventOfCode for Day3 {
    type PuzzleData = Vec<String>;

    fn part_one(puzzle: &Self::PuzzleData) {
        let trees_seen = find_slope_trees(puzzle, 3, 1);
        println!("Part One, trees seen: {}", trees_seen);
    }

    fn part_two(puzzle: &Self::PuzzleData) {
        let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

        let trees_seen = slopes.iter().fold(1, |accumulator, &(slope_x, slope_y)| {
            accumulator * find_slope_trees(puzzle, slope_x, slope_y)
        });
        println!("Part Two, trees seen: {}", trees_seen);
    }

    fn parse_file(puzzle: Vec<String>) -> Self::PuzzleData {
        puzzle
    }
}

fn main() -> anyhow::Result<()> {
    let opts: BasicOptions = BasicOptions::parse();
    Day3::start(PathBuf::from(opts.path_to_puzzles).join("day_3.txt"))
}
