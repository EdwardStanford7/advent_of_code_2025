pub struct Day1;

impl crate::Day for Day1 {
    fn run(input: String) -> crate::DayResult {
        let mut dial = 50;
        let mut hit_0 = 0;
        let mut total = 0;

        for line in input.lines() {
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

        crate::DayResult {
            part_1: hit_0,
            part_2: total as u64,
        }
    }
}
