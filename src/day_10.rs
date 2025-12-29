use good_lp::{Expression, ProblemVariables, Solution, SolverModel, default_solver, variable};

pub struct Day10;

impl crate::Day for Day10 {
    fn run(input: String) -> crate::DayResult {
        let manuals: Vec<MachineManual> = input
            .lines()
            .map(|line| {
                let (lights, rest) = line.trim().split_once(']').unwrap();
                let (buttons, joltages) = rest.split_once('{').unwrap();

                let target_light_pattern = lights
                    .split_at(1)
                    .1
                    .chars()
                    .map(|c| c == '#')
                    .collect::<Vec<bool>>();

                // Parse buttons
                let buttons: Vec<Button> = buttons
                    .trim()
                    .split(' ')
                    .map(|button| {
                        // Split list of buttons into individual buttons
                        let toggles = button
                            .split_at(1)
                            .1
                            .split_at(button.len() - 2)
                            .0
                            .split(',') // For individual button, get each toggle
                            .map(|num| num.parse().unwrap())
                            .collect();

                        Button { toggles }
                    })
                    .collect();

                let joltage_requirements = joltages
                    .trim()
                    .split_at(joltages.len() - 1)
                    .0
                    .split(',')
                    .map(|num| num.parse().unwrap())
                    .collect();

                MachineManual {
                    target_light_pattern,
                    buttons,
                    joltage_requirements,
                }
            })
            .collect();

        let part_1 = manuals
            .iter()
            .fold(0, |acc, manual| acc + manual.min_buttons_lights());

        let part_2 = manuals.iter().fold(0, |acc, manual| {
            acc + manual.calculate_minimum_presses().iter().sum::<u64>()
        });

        crate::DayResult { part_1, part_2 }
    }
}

#[derive(Debug)]
struct MachineManual {
    target_light_pattern: Vec<bool>,
    buttons: Vec<Button>,
    joltage_requirements: Vec<u64>,
}

#[derive(Debug)]
struct Button {
    toggles: Vec<usize>,
}

impl MachineManual {
    pub fn min_buttons_lights(&self) -> u64 {
        // Every button is either pressed once or not at all. Repeat presses just undo everything (mod 2)
        let mut binary_digits = vec![false; self.buttons.len()];
        let mut min_presses = u64::MAX;

        // When all digits are true, we've exhausted all combinations
        while binary_digits.iter().any(|digit| !*digit) {
            if self.check_combination(&binary_digits) {
                let num_presses = binary_digits
                    .iter()
                    .fold(0, |acc, &digit| acc + digit as u64);
                if num_presses < min_presses {
                    min_presses = num_presses;
                }
            }

            Self::increment_binary(&mut binary_digits);
        }

        min_presses
    }

    fn increment_binary(binary_digits: &mut [bool]) {
        binary_digits[0] = !binary_digits[0]; // Flip first bit
        let mut index = 0;
        // Check if we need to carry to next bit
        while !binary_digits[index] {
            index += 1;
            if index >= binary_digits.len() {
                break;
            }
            binary_digits[index] = !binary_digits[index];
        }
    }

    fn check_combination(&self, binary_digits: &[bool]) -> bool {
        let mut lights = vec![false; self.target_light_pattern.len()];

        for (index, digit) in binary_digits.iter().enumerate() {
            if *digit {
                for light in self.buttons[index].toggles.iter() {
                    lights[*light] = !lights[*light];
                }
            }
        }

        lights == self.target_light_pattern
    }

    pub fn calculate_minimum_presses(&self) -> Vec<u64> {
        let mut vars = Vec::new();
        let mut p_vars = ProblemVariables::new();

        for _ in 0..self.buttons.len() {
            vars.push(p_vars.add(variable().min(0).integer()));
        }

        let mut constraints = Vec::new();

        for (index, target) in self.joltage_requirements.iter().enumerate() {
            let mut sum = Expression::default();
            for (button_index, button) in self.buttons.iter().enumerate() {
                if button.toggles.contains(&index) {
                    sum += vars[button_index];
                }
            }

            constraints.push(sum.eq(Expression::from(*target as i32)));
        }

        let mut minimum = Expression::default();
        for var in &vars {
            minimum += *var;
        }

        let mut solver = p_vars.minimise(minimum).using(default_solver);

        for constraint in constraints.clone() {
            solver = solver.with(constraint);
        }

        let solution = solver.solve().unwrap();

        let mut result = Vec::new();
        for var in vars {
            result.push(solution.value(var) as u64);
        }

        result
    }
}
