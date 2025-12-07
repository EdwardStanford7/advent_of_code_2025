use rangetools::Rangetools;

pub struct Day5;

impl crate::Day for Day5 {
    fn run(input: String) -> crate::DayResult {
        let (ranges, ids) = input
            .split_once("\n\n")
            .expect("Input not formatted correctly");

        let ranges: Vec<_> = ranges
            .lines()
            .map(|line| {
                let (start, end) = line
                    .trim()
                    .split_once("-")
                    .expect("Input not formatted correctly");
                let start: u64 = start.parse().expect("Failed to parse range start");
                let end: u64 = end.parse().expect("Failed to parse range end");
                start..=end
            })
            .collect();

        let full_range = ranges
            .iter()
            .skip(2)
            .fold(ranges[0].clone().union(ranges[1].clone()), |acc, range| {
                acc.union(range.clone())
            });

        let ids: Vec<u64> = ids
            .lines()
            .map(|line| line.trim().parse().expect("Failed to parse ID"))
            .collect();

        crate::DayResult {
            part_1: ids.iter().filter(|id| full_range.contains(**id)).count() as u64,
            part_2: full_range.into_iter().count() as u64,
        }
    }
}
