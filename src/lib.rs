use std::fs::File;
use clap::Clap;
use std::path::Path;
use std::io::{BufReader, BufRead};

#[derive(Clone, Debug, Default, Clap)]
#[clap(version = "1.0")]
pub struct BasicOptions {
    #[clap(short, long, default_value = "puzzle_input/")]
    pub path_to_puzzles: String,
}

pub fn read_puzzle_file(file_path: impl AsRef<Path>) -> anyhow::Result<Vec<String>> {
    let puzzle_file = BufReader::new(File::open(file_path)?);
    Ok(puzzle_file.lines().filter_map(Result::ok).collect())
}