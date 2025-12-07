pub struct Day6;

impl crate::Day for Day6 {
    fn run(input: String) -> crate::DayResult {
        let lines = input
            .lines()
            .collect::<Vec<&str>>()
            .iter()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect::<Vec<Vec<char>>>();

        let mut problems = Vec::new();
        let mut current_operator = Operator::Add; // Placeholder
        let mut current_digit_grid = vec![Vec::new(); lines.len() - 1];
        for column in 0..lines[0].len() {
            let mut current_column = Vec::new();
            for line in &lines {
                current_column.push(line[column]);
            }

            if current_column.iter().filter(|c| **c != ' ').count() == 0 {
                problems.push(CephalopodProblem::new(
                    current_digit_grid.clone(),
                    current_operator.clone(),
                ));
                current_digit_grid = vec![Vec::new(); lines.len() - 1];
            } else {
                for row in 0..(lines.len() - 1) {
                    current_digit_grid[row].push(current_column[row]);
                }
                if current_column[lines.len() - 1] == '+' {
                    current_operator = Operator::Add;
                } else if current_column[lines.len() - 1] == '*' {
                    current_operator = Operator::Multiply;
                }
            }
        }
        problems.push(CephalopodProblem::new(current_digit_grid, current_operator));

        crate::DayResult {
            part_1: problems
                .iter()
                .map(|col| col.compute_horizontal())
                .sum::<u64>(),
            part_2: problems
                .iter()
                .map(|col| col.compute_vertical())
                .sum::<u64>(),
        }
    }
}

#[derive(Debug)]
struct CephalopodProblem {
    horizontal: Vec<u64>,
    vertical: Vec<u64>,
    operator: Operator,
}

impl CephalopodProblem {
    fn new(digit_grid: Vec<Vec<char>>, operator: Operator) -> Self {
        let horizontal = Self::parse_horizontal(digit_grid.clone());
        let vertical = Self::parse_vertical(digit_grid);

        CephalopodProblem {
            horizontal,
            vertical,
            operator,
        }
    }

    fn parse_vertical(digit_grid: Vec<Vec<char>>) -> Vec<u64> {
        let mut result = Vec::new();
        for col_idx in 0..digit_grid[0].len() {
            let mut col_chars = Vec::new();
            for row in &digit_grid {
                col_chars.push(row[col_idx]);
            }
            let col_number = col_chars
                .iter()
                .collect::<String>()
                .trim()
                .parse::<u64>()
                .unwrap();
            result.push(col_number);
        }
        result
    }

    fn parse_horizontal(digit_grid: Vec<Vec<char>>) -> Vec<u64> {
        digit_grid
            .iter()
            .map(|row| {
                row.iter()
                    .collect::<String>()
                    .trim()
                    .parse::<u64>()
                    .unwrap()
            })
            .collect()
    }

    fn compute_horizontal(&self) -> u64 {
        match self.operator {
            Operator::Add => self.horizontal.iter().sum(),
            Operator::Multiply => self.horizontal.iter().product(),
        }
    }

    fn compute_vertical(&self) -> u64 {
        match self.operator {
            Operator::Add => self.vertical.iter().sum(),
            Operator::Multiply => self.vertical.iter().product(),
        }
    }
}

#[derive(Debug, Clone)]
enum Operator {
    Add,
    Multiply,
}
