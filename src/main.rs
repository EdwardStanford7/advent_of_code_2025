fn main() {
    day_1();
}

fn day_1() {
    let mut dial_val = 50;
    let mut num_0s = 0;

    let instructions =
        std::fs::read_to_string("inputs/day_1.txt").expect("Failed to read input file");

    for line in instructions.lines() {
        let (direction, value_str) = line.split_at(1);
        let value: i32 = value_str.trim().parse().expect("Failed to parse value");

        if direction == "L" {
            if dial_val == 0 {
                dial_val = 100;
            }
            num_0s += (dial_val - value - 100) / (-100);
            dial_val -= value;
        } else if direction == "R" {
            num_0s += (dial_val + value) / 100;
            dial_val += value;
        }

        dial_val = ((dial_val % 100) + 100) % 100; // keep dial_val in [0, 99]
    }

    println!("Day 1: {num_0s}");
}
