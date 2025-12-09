use std::{collections::HashMap, hash::Hash};

pub struct Day8;

impl crate::Day for Day8 {
    fn run(input: String) -> crate::DayResult {
        let mut junctions = input
            .lines()
            .map(|line| {
                line.trim()
                    .split(',')
                    .map(|num| num.parse().unwrap())
                    .collect::<Vec<i32>>()
            })
            .map(|coord| JunctionBox::new(coord[0], coord[1], coord[2]))
            .collect::<Vec<JunctionBox>>();

        let mut circuits = HashMap::<usize, usize>::new();
        let mut circuit_id = 0;
        let mut part1 = 0;
        let mut last_pair = (JunctionBox::new(0, 0, 0), JunctionBox::new(0, 0, 0));

        for (index, pair) in closest_pairs(&junctions).iter().enumerate() {
            let (i, j) = *pair;

            // Two unconnected junctions
            if junctions[i].circuit_id.is_none() && junctions[j].circuit_id.is_none() {
                junctions[i].circuit_id = Some(circuit_id);
                junctions[j].circuit_id = Some(circuit_id);
                circuits.insert(circuit_id, 2);
                circuit_id += 1;
            }
            // First in a circuit already, second unconnected
            else if let Some(i_id) = junctions[i].circuit_id {
                if let Some(j_id) = junctions[j].circuit_id {
                    // Second in a different circuit, merge the two circuits
                    if i_id != j_id {
                        let to_merge = j_id;
                        // Update all junctions in the merged circuit to the new circuit ID
                        for junction in junctions.iter_mut() {
                            if junction.circuit_id == Some(to_merge) {
                                junction.circuit_id = Some(i_id);
                            }
                        }

                        // Update size of the merged circuit
                        let count = circuits.remove(&to_merge).unwrap();
                        *circuits.get_mut(&i_id).unwrap() += count;
                    }
                } else {
                    junctions[j].circuit_id = Some(i_id);
                    *circuits.get_mut(&i_id).unwrap() += 1;
                }
            }
            // First unconnected, second in a circuit
            else if let Some(j_id) = junctions[j].circuit_id {
                junctions[i].circuit_id = Some(j_id);
                *circuits.get_mut(&j_id).unwrap() += 1;
            }

            // Part 1 is make exactly 1000 connections
            if index == 999 {
                let mut temp: Vec<_> = circuits.values().collect();
                temp.sort_by(|a, b| b.cmp(a));
                part1 = temp.iter().take(3).copied().product::<usize>() as u64;
            }

            // Part 2 is when all junctions are connected
            if circuits.len() == 1 && *circuits.iter().max().unwrap().1 == junctions.len() {
                last_pair = (junctions[i].clone(), junctions[j].clone());
                break;
            }
        }

        crate::DayResult {
            part_1: part1,
            part_2: (last_pair.0.x * last_pair.1.x) as u64,
        }
    }
}

// Order all possible pairs in the list by distance
fn closest_pairs(junctions: &[JunctionBox]) -> Vec<(usize, usize)> {
    let mut distances = Vec::new();

    for (i, j1) in junctions.iter().enumerate() {
        for (k, j2) in junctions.iter().enumerate().skip(i + 1) {
            let distance = j1.distance(j2);
            distances.push((distance, i, k));
        }
    }

    distances.sort_by(|(d1, _, _), (d2, _, _)| d1.partial_cmp(d2).unwrap());

    distances.into_iter().map(|(_, i, k)| (i, k)).collect()
}

#[derive(Eq, PartialEq, Hash, Debug, Clone)]
struct JunctionBox {
    x: i32,
    y: i32,
    z: i32,
    circuit_id: Option<usize>,
}

impl JunctionBox {
    pub fn new(x: i32, y: i32, z: i32) -> JunctionBox {
        JunctionBox {
            x,
            y,
            z,
            circuit_id: None,
        }
    }

    pub fn distance(&self, other: &JunctionBox) -> f32 {
        f32::sqrt(
            f32::powi((self.x - other.x) as f32, 2)
                + f32::powi((self.y - other.y) as f32, 2)
                + f32::powi((self.z - other.z) as f32, 2),
        )
    }
}
