use clap::Clap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

pub trait AdventOfCode {
    type PuzzleData;

    fn start(puzzle_path: impl AsRef<Path>) -> anyhow::Result<()> {
        let parsed_data = Self::parse_file(read_puzzle_file(puzzle_path)?);

        println!("Starting part one!");
        Self::part_one(&parsed_data);
        println!("Starting part two!");
        Self::part_two(&parsed_data);
        println!("Finished");
        Ok(())
    }

    fn part_one(puzzle: &Self::PuzzleData);
    fn part_two(puzzle: &Self::PuzzleData);
    fn parse_file(puzzle: Vec<String>) -> Self::PuzzleData;
}

#[derive(Clone, Debug, Default, Clap)]
#[clap(version = "1.0")]
pub struct BasicOptions {
    #[clap(short, long, default_value = "puzzle_input/")]
    pub path_to_puzzles: String,
}

pub fn read_puzzle_file(file_path: impl AsRef<Path>) -> anyhow::Result<Vec<String>> {
    let puzzle_file = BufReader::new(File::open(file_path)?);
    Ok(puzzle_file.lines().flatten().collect())
}
