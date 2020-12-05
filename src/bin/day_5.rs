use std::path::PathBuf;

use clap::Clap;

use advent_of_code_2020::{AdventOfCode, BasicOptions};
use itertools::Itertools;

struct Day5;

fn binary_space(chars: impl Iterator<Item=char>, min_max: (usize, usize)) -> usize {
    chars.fold(min_max, |(lower_bound, upper_bound), c| {
        // Ceiling divide.
        let delta = ((upper_bound - lower_bound) + 1) / 2;
        match c {
            'F' | 'L' => (lower_bound, upper_bound - delta),
            'B' | 'R' => (lower_bound + delta, upper_bound),
            _ => unreachable!()
        }
    }).0
}

struct Seat {
    row: usize,
    column: usize,
}

impl Seat {
    pub fn new(code: impl AsRef<str>) -> Self {
        let row = binary_space(code.as_ref().chars().take(7), (0, 127));
        let column = binary_space(code.as_ref().chars().skip(7).take(3), (0, 7));
        Seat {
            row,
            column
        }
    }

    pub fn seat_id(self) -> usize {
        self.row * 8 + self.column
    }
}

impl AdventOfCode for Day5 {
    type PuzzleData = Vec<usize>;

    fn part_one(puzzle: &Self::PuzzleData) {
        let max_id = puzzle.iter().max().unwrap();
        println!("Max id: {}", max_id)
    }

    fn part_two(puzzle: &Self::PuzzleData) {
        let missing_seat = puzzle
            .windows(2)
            .find(|values| values[0] + 1 != values[1])
            .unwrap();
        println!("Missing seat: {}", missing_seat[0] + 1)
    }

    fn parse_file(puzzle: Vec<String>) -> Self::PuzzleData {
        puzzle.into_iter().map(Seat::new).map(Seat::seat_id).sorted().collect()
    }
}

fn main() -> anyhow::Result<()> {
    let opts: BasicOptions = BasicOptions::parse();
    Day5::start(PathBuf::from(opts.path_to_puzzles).join("day_5.txt"))
}

#[cfg(test)]
mod tests {
    use crate::Seat;

    #[test]
    pub fn parse_test() {
        let seat = Seat::new("FBFBBFFRLR");
        assert_eq!(seat.row, 44);
        assert_eq!(seat.column, 5);
    }
}
