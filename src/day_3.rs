use crate::memoizer::{HashRef, Memoizer};
type MyMemoizer<'a> = Memoizer<(HashRef<'a, [u8]>, u8), u64>;

pub struct Day3;

impl crate::Day for Day3 {
    fn run(input: String) -> crate::DayResult {
        let battery_banks = input;

        let mut total_joltage_part_1: u64 = 0;
        let mut total_joltage_part_2: u64 = 0;

        for bank in battery_banks.lines() {
            let batteries = bank
                .chars()
                .map(|digit| {
                    digit
                        .to_digit(10)
                        .expect("couldn't convert input to number") as u8
                })
                .collect::<Vec<u8>>();

            let max_part_1 =
                largest_ordered_digits(batteries.as_slice(), 2, &mut MyMemoizer::new());
            total_joltage_part_1 += max_part_1;

            let max_part_2 =
                largest_ordered_digits(batteries.as_slice(), 12, &mut MyMemoizer::new());
            total_joltage_part_2 += max_part_2;
        }

        crate::DayResult {
            part_1: total_joltage_part_1,
            part_2: total_joltage_part_2,
        }
    }
}

fn largest_ordered_digits<'a>(
    digits: &'a [u8],
    num_digits: u8,
    memo_table: &mut MyMemoizer<'a>,
) -> u64 {
    let memo_key = (HashRef { _ref: digits }, num_digits);

    if let Some(cached) = memo_table.get(&memo_key) {
        return *cached;
    }

    if num_digits == 1 {
        return *digits.iter().max().unwrap() as u64;
    }

    let mut max = 0;
    for i in 0..(digits.len() - (num_digits as usize) + 1) {
        let max_from_here = digits[i] as u64 * 10u64.pow(num_digits as u32 - 1)
            + largest_ordered_digits(&digits[(i + 1)..], num_digits - 1, memo_table);
        if max_from_here > max {
            max = max_from_here;
        }
    }

    memo_table.insert(memo_key, max);

    max
}
