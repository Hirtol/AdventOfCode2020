use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::option::Option::Some;
use std::path::PathBuf;

use clap::Clap;
use itertools::Itertools;

use advent_of_code_2020::{read_puzzle_file, AdventOfCode, BasicOptions};

struct Day4;

fn validate_fields(map: &HashMap<String, String>) -> bool {
    map.iter().all(|(key, value)| match key.as_str() {
        "byr" => (1920..=2002).contains(&value.parse::<usize>().unwrap_or_default()),
        "iyr" => (2010..=2020).contains(&value.parse::<usize>().unwrap_or_default()),
        "eyr" => (2020..=2030).contains(&value.parse::<usize>().unwrap_or_default()),
        "hgt" => {
            if value.contains("cm") {
                let height: usize = value.split_at(value.find("cm").unwrap()).0.parse().unwrap_or_default();
                (150..=193).contains(&height)
            } else if value.contains("in") {
                let height: usize = value.split_at(value.find("in").unwrap()).0.parse().unwrap_or_default();
                (59..=76).contains(&height)
            } else {
                false
            }
        }
        "hcl" => value.starts_with("#") && hex::decode(value.chars().skip(1).take(6).collect::<String>()).is_ok(),
        "ecl" => {
            const EYE_COLOUR: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
            EYE_COLOUR.contains(&value.as_str())
        }
        "pid" => value.chars().count() == 9 && value.parse::<u64>().is_ok(),
        "cid" => true,
        _ => unreachable!(),
    })
}

fn count_valid(puzzle: &<Day4 as AdventOfCode>::PuzzleData, check_field_validity: bool) -> usize {
    puzzle
        .iter()
        .filter(|map| {
            (!check_field_validity | validate_fields(map)) && map.len() == if map.contains_key("cid") { 8 } else { 7 }
        })
        .count()
}

impl AdventOfCode for Day4 {
    type PuzzleData = Vec<HashMap<String, String>>;

    fn part_one(puzzle: &Self::PuzzleData) {
        println!("Valid passwords: {}", count_valid(puzzle, false));
    }

    fn part_two(puzzle: &Self::PuzzleData) {
        println!("Valid passwords: {}", count_valid(puzzle, true));
    }

    fn parse_file(puzzle: Vec<String>) -> Self::PuzzleData {
        let chunks = puzzle.split(|line| line.is_empty());
        chunks
            .into_iter()
            .map(|line_1| {
                line_1
                    .join(" ")
                    .split_whitespace()
                    .map(|key_value| {
                        let mut split_iter = key_value.split(":");
                        let mut get_next = || split_iter.next().unwrap().to_string();
                        (get_next(), get_next())
                    })
                    .collect::<HashMap<String, String>>()
            })
            .collect()
    }
}

fn main() -> anyhow::Result<()> {
    let opts: BasicOptions = BasicOptions::parse();
    Day4::start(PathBuf::from(opts.path_to_puzzles).join("day_4.txt"))
}
