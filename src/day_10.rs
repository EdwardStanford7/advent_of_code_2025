use crate::memoizer::Memoizer;

pub struct Day10;

impl crate::Day for Day10 {
    fn run(input: String) -> crate::DayResult {
        let manuals: Vec<MachineManual> = input
            .lines()
            .map(|line| {
                let (lights, rest) = line.trim().split_once(']').unwrap();
                let (buttons, joltages) = rest.split_once('{').unwrap();

                let indicator_lights = lights
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
                    target_light_pattern: indicator_lights,
                    buttons,
                    joltage_requirements,
                }
            })
            .collect();

        let part_1 = manuals
            .iter()
            .fold(0, |acc, manual| acc + manual.min_buttons_lights());

        let part_2 = manuals.iter().fold(0, |acc, manual| {
            println!("Finished machine {:?}", manual);
            acc + manual.min_buttons_joltages()
        });

        // println!(
        //     "Min presses for machine 0: {}",
        //     manuals[0].min_buttons_joltages()
        // );
        // println!(
        //     "Min presses for machine 1: {}",
        //     manuals[1].min_buttons_joltages()
        // );
        // println!(
        //     "Min presses for machine 2: {}",
        //     manuals[2].min_buttons_joltages()
        // );

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
                    // println!(
                    //     "Combination {:?} takes {} presses",
                    //     binary_digits, num_presses
                    // );
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

    pub fn min_buttons_joltages(&self) -> u64 {
        let mut memo_table = Memoizer::new();
        self.calculate_min_buttons(vec![0; self.joltage_requirements.len()], &mut memo_table)
    }

    // Dynamic programming approach
    fn calculate_min_buttons(
        &self,
        current_joltages: Vec<u64>,
        memo_table: &mut Memoizer<Vec<u64>, u64>,
    ) -> u64 {
        if let Some(cached_result) = memo_table.get(&current_joltages) {
            return *cached_result;
        }

        // println!("{:?}", current_joltages);

        if current_joltages
            .iter()
            .enumerate()
            .all(|(index, joltage)| joltage == &self.joltage_requirements[index])
        {
            return 0;
        }

        let mut min_presses = u64::MAX;
        for button in self.buttons.iter() {
            let new_joltages = Self::apply_joltages(current_joltages.clone(), button);
            // If this button would exceed joltages, skip it
            if self.check_exceeded_joltages(&new_joltages) {
                continue;
            }

            let recursive_solution = self.calculate_min_buttons(new_joltages.clone(), memo_table);
            // If there was no valid solution from this state, skip it
            if recursive_solution == u64::MAX {
                continue;
            }

            // Add 1 for the button we just pressed
            let current_solution = recursive_solution + 1;
            if current_solution < min_presses {
                min_presses = current_solution;
            }
        }

        memo_table.insert(current_joltages, min_presses);

        min_presses
    }

    fn apply_joltages(mut joltages: Vec<u64>, button: &Button) -> Vec<u64> {
        for &toggle in &button.toggles {
            joltages[toggle] += 1;
        }
        joltages
    }

    fn check_exceeded_joltages(&self, joltages: &[u64]) -> bool {
        for (i, &joltage) in joltages.iter().enumerate() {
            if joltage > self.joltage_requirements[i] {
                return true;
            }
        }
        false
    }
}
