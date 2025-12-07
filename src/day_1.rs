pub fn run() {
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
