use std::collections::HashMap;

fn main() {
    println!("Advent of Code 2025");
    // day_1();
    // day_2();
    day_3();
}

fn day_1() {
    let mut dial = 50;
    let mut hit_0 = 0;
    let mut total = 0;

    let instructions =
        std::fs::read_to_string("inputs/day_1.txt").expect("Failed to read input file");

    for line in instructions.lines() {
        let (direction, value_str) = line.split_at(1);
        let change: i32 = value_str.trim().parse().expect("Failed to parse value");

        if direction == "L" {
            if dial == 0 {
                dial = 100;
            }
            total += (dial - change - 100) / (-100);
            dial -= change;
        } else if direction == "R" {
            total += (dial + change) / 100;
            dial += change;
        }
        dial = ((dial % 100) + 100) % 100; // keep dial_val in [0, 99]
        if dial == 0 {
            hit_0 += 1;
        }
    }

    println!("Day 1: \n\tPart 1 {hit_0}\n\tPart 2 {total}");
}

fn day_2() {
    fn is_invalid_id_part_1(id: String) -> bool {
        if id.len() % 2 != 0 {
            return false;
        }

        let (first_half, second_half) = id.split_at(id.len() / 2);
        first_half == second_half
    }

    fn is_invalid_id_part_2(id: String) -> bool {
        for factor in 2..=id.len() {
            if id.len() % factor != 0 {
                continue;
            }
            let substring_length = id.len() / factor;
            let pattern = &id[0..substring_length];

            let mut pattern_matches = true;
            for i in 1..factor {
                let substring =
                    &id[(substring_length * i)..(substring_length * i + substring_length)];
                if substring != pattern {
                    pattern_matches = false;
                }
            }
            if pattern_matches {
                return true;
            }
        }
        false
    }

    let input = std::fs::read_to_string("inputs/day_2.txt").expect("Failed to read input file");
    let id_ranges = input.split(",");

    let mut id_sum_part_1 = 0;
    let mut id_sum_part_2 = 0;

    for range in id_ranges {
        let (start, end) = range
            .trim()
            .split_once("-")
            .expect("Failed to parse ID range");

        let start_id: u64 = start.parse().expect("Failed to parse start ID");
        let end_id: u64 = end.parse().expect("Failed to parse end ID");

        for id in start_id..=end_id {
            if is_invalid_id_part_1(id.to_string()) {
                id_sum_part_1 += id;
            }
            if is_invalid_id_part_2(id.to_string()) {
                id_sum_part_2 += id;
            }
        }
    }

    println!("Day 2: \n\tPart 1 {id_sum_part_1}\n\tPart 2 {id_sum_part_2}");
}

fn day_3() {
    fn largest_ordered_digits(
        digits: &[u64],
        num_digits: u8,
        memo_table: &mut HashMap<(*const [u64], u8), u64>,
    ) -> u64 {
        if let Some(cached) = memo_table.get(&(digits, num_digits)) {
            return *cached;
        }

        if num_digits == 1 {
            return *digits.iter().max().unwrap();
        }

        let mut max = 0;
        for i in 0..(digits.len() - (num_digits as usize) + 1) {
            let max_from_here = digits[i] * 10u64.pow(num_digits as u32 - 1)
                + largest_ordered_digits(&digits[(i + 1)..], num_digits - 1, memo_table);
            if max_from_here > max {
                max = max_from_here;
            }
        }

        memo_table.insert((digits, num_digits), max);

        max
    }

    let battery_banks =
        std::fs::read_to_string("inputs/day_3.txt").expect("Failed to read input file");

    let mut total_joltage_part_1: u64 = 0;
    let mut total_joltage_part_2: u64 = 0;

    for bank in battery_banks.lines() {
        let batteries = bank
            .chars()
            .map(|digit| {
                digit
                    .to_digit(10)
                    .expect("couldn't convert input to number") as u64
            })
            .collect::<Vec<u64>>();

        let max_part_1 = largest_ordered_digits(batteries.as_slice(), 2, &mut HashMap::new());
        total_joltage_part_1 += max_part_1;

        let max_part_2 = largest_ordered_digits(batteries.as_slice(), 12, &mut HashMap::new());
        total_joltage_part_2 += max_part_2;
    }

    println!("Day 3: \n\tPart 1 {total_joltage_part_1}\n\tPart 2 {total_joltage_part_2}");
}
