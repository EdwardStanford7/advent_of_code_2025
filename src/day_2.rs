pub struct Day2;

impl crate::Day for Day2 {
    fn run(input: String) -> crate::DayResult {
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

        crate::DayResult {
            part_1: id_sum_part_1,
            part_2: id_sum_part_2,
        }
    }
}

fn is_invalid_id_part_1(id: String) -> bool {
    if !id.len().is_multiple_of(2) {
        return false;
    }

    let (first_half, second_half) = id.split_at(id.len() / 2);
    first_half == second_half
}

fn is_invalid_id_part_2(id: String) -> bool {
    for factor in 2..=id.len() {
        if !id.len().is_multiple_of(factor) {
            continue;
        }
        let substring_length = id.len() / factor;
        let pattern = &id[0..substring_length];

        let mut pattern_matches = true;
        for i in 1..factor {
            let substring = &id[(substring_length * i)..(substring_length * i + substring_length)];
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
