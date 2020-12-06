use advent_of_code_2020::{AdventOfCode, BasicOptions};
use clap::Clap;
use itertools::Itertools;
use std::path::PathBuf;

type GroupAnswers = String;
struct Day6;

impl AdventOfCode for Day6 {
    type PuzzleData = Vec<Vec<GroupAnswers>>;

    fn part_one(puzzle: &Self::PuzzleData) {
        let total = puzzle.iter()
            .map(|lines| lines.join(""))
            .map(|answers| answers.chars().unique().count())
            .sum::<usize>();
        println!("Sum of unique answers: {}", total);
    }

    fn part_two(puzzle: &Self::PuzzleData) {
        let total = puzzle.iter()
            .map(|lines| {
                let mut iter = lines.iter();
                let first = iter.next().unwrap().clone();
                lines.iter().fold(first, |acc, answer| {
                    acc.chars().filter(|&c| answer.contains(c)).collect::<GroupAnswers>()
                })
            })
            .map(|answers| answers.len())
            .sum::<usize>();
        println!("Sum of common answers: {}", total);
    }

    fn parse_file(puzzle: Vec<String>) -> Self::PuzzleData {
        let chunks = puzzle.split(|line| line.is_empty());
        chunks
            .into_iter()
            .map(|lines| lines.to_vec())
            .collect()
    }
}

fn main() -> anyhow::Result<()> {
    let opts: BasicOptions = BasicOptions::parse();
    Day6::start(PathBuf::from(opts.path_to_puzzles).join("day_6.txt"))
}
