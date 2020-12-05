use advent_of_code_2020::{AdventOfCode, BasicOptions};
use clap::Clap;
use itertools::Itertools;
use std::path::PathBuf;

type SeatId = usize;
struct Day5;

struct Seat {
    row: usize,
    column: usize,
}

impl Seat {
    pub fn new(code: impl AsRef<str>) -> Self {
        Seat {
            row: binary_space(code.as_ref().chars().take(7), (0, 127)),
            column: binary_space(code.as_ref().chars().skip(7).take(3), (0, 7)),
        }
    }

    pub fn seat_id(self) -> SeatId {
        self.row * 8 + self.column
    }
}

fn binary_space(chars: impl Iterator<Item = char>, min_max: (usize, usize)) -> usize {
    chars
        .fold(min_max, |(lower_bound, upper_bound), c| {
            // Ceiling divide.
            let delta = ((upper_bound - lower_bound) + 1) / 2;
            match c {
                'F' | 'L' => (lower_bound, upper_bound - delta),
                'B' | 'R' => (lower_bound + delta, upper_bound),
                _ => unreachable!(),
            }
        })
        .0
}

impl AdventOfCode for Day5 {
    type PuzzleData = Vec<SeatId>;

    fn part_one(puzzle: &Self::PuzzleData) {
        println!("Max id: {}", puzzle.iter().max().unwrap())
    }

    fn part_two(puzzle: &Self::PuzzleData) {
        let missing_seat = puzzle.windows(2).find(|values| values[0] + 1 != values[1]).unwrap();
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
    #[test]
    pub fn parse_test() {
        let seat = super::Seat::new("FBFBBFFRLR");
        assert_eq!(seat.row, 44);
        assert_eq!(seat.column, 5);
    }
}
