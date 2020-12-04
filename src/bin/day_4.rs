use std::collections::HashMap;
use std::ops::RangeInclusive;
use std::option::Option::Some;
use std::path::PathBuf;

use clap::Clap;
use itertools::Itertools;

use advent_of_code_2020::{read_puzzle_file, AdventOfCode, BasicOptions};

struct Day4;

fn validate_fields(map: &HashMap<String, String>) -> bool {
    map.iter().all(|(key, value)| {
        let values = match key.as_str() {
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
        };
        values
    })
}

impl AdventOfCode for Day4 {
    type PuzzleData = Vec<HashMap<String, String>>;

    fn part_one(puzzle: &Self::PuzzleData) {
        let valid_amount = puzzle
            .iter()
            .filter(|map| {
                if !map.contains_key("cid") {
                    map.len() == 7
                } else {
                    map.len() == 8
                }
            })
            .count();
        println!("Valid passwords: {}", valid_amount);
    }

    fn part_two(puzzle: &Self::PuzzleData) {
        let valid_amount = puzzle
            .iter()
            .filter(|map| {
                if !map.contains_key("cid") {
                    map.len() == 7 && validate_fields(map)
                } else {
                    map.len() == 8 && validate_fields(map)
                }
            })
            .count();
        println!("Valid passwords: {}", valid_amount);
    }

    fn parse_file(puzzle: Vec<String>) -> Self::PuzzleData {
        let chunks = puzzle.split(|line| line.is_empty());
        chunks.into_iter()
            .map(|line_1| {
                line_1
                    .join(" ")
                    .split_whitespace()
                    .map(|key_value| {
                        let (key, value) = key_value.split_at(key_value.find(':').unwrap());
                        (key.to_string(), value.trim_start_matches(":").to_string())
                    })
                    .collect()
            })
            .collect()
    }
}

fn main() -> anyhow::Result<()> {
    let opts: BasicOptions = BasicOptions::parse();
    Day4::start(PathBuf::from(opts.path_to_puzzles).join("day_4.txt"))
}
