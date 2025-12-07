pub struct Day7;

impl crate::Day for Day7 {
    fn run(input: String) -> crate::DayResult {
        let mut tachyon_manifold =
            TachyonManifold::new(input.lines().map(|line| line.chars().collect()).collect());

        let (splits, timelines) = tachyon_manifold.run_beam();

        crate::DayResult {
            part_1: splits,
            part_2: timelines,
        }
    }
}

struct TachyonManifold {
    grid: Vec<Vec<GridSpace>>,
}

impl TachyonManifold {
    fn new(input: Vec<Vec<char>>) -> Self {
        TachyonManifold {
            grid: input
                .into_iter()
                .map(|row| {
                    row.into_iter()
                        .map(|c| match c {
                            '.' => GridSpace::Empty,
                            '^' => GridSpace::Splitter,
                            'S' => GridSpace::TachyonBeam(1),
                            _ => panic!("Invalid character in input"),
                        })
                        .collect()
                })
                .collect(),
        }
    }

    // Runs the tachyon beam through the manifold, returning the number of splits and timelines.
    fn run_beam(&mut self) -> (u64, u64) {
        let mut num_splits = 0;
        for i in 0..(self.grid.len() - 1) {
            let (first, second) = self.grid.split_at_mut(i + 1);
            num_splits += Self::process_row(&first[i], &mut second[0]);
        }

        (
            num_splits,
            self.grid.last().unwrap().iter().fold(0, |acc, space| {
                if let GridSpace::TachyonBeam(timelines) = space {
                    acc + timelines
                } else {
                    acc
                }
            }),
        )
    }

    fn process_row(current_row: &[GridSpace], next_row: &mut [GridSpace]) -> u64 {
        let mut splits = 0;
        for (index, space) in current_row.iter().enumerate() {
            if let GridSpace::TachyonBeam(timelines) = space {
                // Check the space below
                match &mut next_row[index] {
                    GridSpace::Empty => {
                        // Move the beam down
                        next_row[index] = GridSpace::TachyonBeam(*timelines);
                    }
                    GridSpace::Splitter => {
                        // Add the timelines to the left
                        if let GridSpace::TachyonBeam(existing_timelines) = &mut next_row[index - 1]
                        {
                            *existing_timelines += timelines;
                        } else {
                            next_row[index - 1] = GridSpace::TachyonBeam(*timelines);
                        }

                        // Add to the right
                        if let GridSpace::TachyonBeam(existing_timelines) = &mut next_row[index + 1]
                        {
                            *existing_timelines += timelines;
                        } else {
                            next_row[index + 1] = GridSpace::TachyonBeam(*timelines);
                        }

                        splits += 1;
                    }
                    GridSpace::TachyonBeam(existing_timelines) => {
                        // Just add the timelines to the existing beam
                        *existing_timelines += timelines;
                    }
                }
            }
        }
        splits
    }
}
#[derive(Clone, Debug)]
enum GridSpace {
    Empty,
    TachyonBeam(u64), // Number of timelines in this space
    Splitter,
}
