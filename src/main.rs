mod day_1;
mod day_10;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod day_6;
mod day_7;
mod day_8;
mod day_9;
mod memoizer;

use day_1::Day1;
use day_2::Day2;
use day_3::Day3;
use day_4::Day4;
use day_5::Day5;
use day_6::Day6;
use day_7::Day7;
use day_8::Day8;
use day_9::Day9;
use day_10::Day10;

pub trait Day {
    fn run(input: String) -> DayResult;
}

pub struct DayResult {
    pub part_1: u64,
    pub part_2: u64,
}

struct DayInfo {
    day: u8,
    run_fn: fn(String) -> DayResult,
}

impl DayInfo {
    fn new(day: u8, run_fn: fn(String) -> DayResult) -> Self {
        DayInfo { day, run_fn }
    }
}

fn main() {
    println!("Advent of Code 2025");

    let days: &[DayInfo] = &[
        DayInfo::new(1, Day1::run),
        DayInfo::new(2, Day2::run),
        DayInfo::new(3, Day3::run),
        DayInfo::new(4, Day4::run),
        DayInfo::new(5, Day5::run),
        DayInfo::new(6, Day6::run),
        DayInfo::new(7, Day7::run),
        DayInfo::new(8, Day8::run),
        DayInfo::new(9, Day9::run),
        DayInfo::new(10, Day10::run),
    ];

    for day_info in days {
        let input = std::fs::read_to_string(format!("inputs/day_{}.txt", day_info.day))
            .expect("Failed to read input file");
        let result = (day_info.run_fn)(input);
        println!(
            "Day {}:\n\tPart 1: {}\n\tPart 2: {}",
            day_info.day, result.part_1, result.part_2
        );
    }
}
