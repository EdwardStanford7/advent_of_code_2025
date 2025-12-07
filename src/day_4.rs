pub struct Day4;

impl crate::Day for Day4 {
    fn run(input: String) -> crate::DayResult {
        let mut grid = PaperRollGrid::new(input);
        crate::DayResult {
            part_1: grid.count_accessible_rolls(),
            part_2: grid.remove_rolls(),
        }
    }
}

struct PaperRollGrid {
    height: usize,
    width: usize,
    grid: Vec<Vec<bool>>, //Is there a roll of paper at this position or not.
}

impl PaperRollGrid {
    pub fn new(input: String) -> Self {
        let grid: Vec<Vec<bool>> = input.lines().map(Self::process_line).collect();
        let height = grid.len();
        let width = if height > 0 { grid[0].len() } else { 0 };

        PaperRollGrid {
            height,
            width,
            grid,
        }
    }

    fn process_line(line: &str) -> Vec<bool> {
        line.trim().chars().map(|c| c == '@').collect()
    }

    pub fn remove_rolls(&mut self) -> u64 {
        let mut removed_rolls = 0;
        while self.count_accessible_rolls() > 0 {
            for y in 0..self.height {
                for x in 0..self.width {
                    if self.grid[y][x] && self.count_adjacent_rolls(x, y) < 4 {
                        self.grid[y][x] = false;
                        removed_rolls += 1;
                    }
                }
            }
        }
        removed_rolls
    }

    pub fn count_accessible_rolls(&self) -> u64 {
        let mut accessible_count = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if self.grid[y][x] && self.count_adjacent_rolls(x, y) < 4 {
                    accessible_count += 1;
                }
            }
        }
        accessible_count
    }

    fn count_adjacent_rolls(&self, x: usize, y: usize) -> u8 {
        [
            (-1, -1),
            (-1, 0),
            (-1, 1),
            (0, -1),
            (0, 1),
            (1, -1),
            (1, 0),
            (1, 1),
        ]
        .map(|(dx, dy)| {
            if self.check_direction(x, y, dx, dy) {
                1
            } else {
                0
            }
        })
        .iter()
        .sum()
    }

    fn check_direction(&self, x: usize, y: usize, dx: isize, dy: isize) -> bool {
        let new_x = x as isize + dx;
        let new_y = y as isize + dy;

        if new_x < 0 || new_x >= self.width as isize || new_y < 0 || new_y >= self.height as isize {
            return false;
        }

        self.grid[new_y as usize][new_x as usize]
    }
}
