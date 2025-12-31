use std::{collections::HashMap, hash::Hash};

pub struct Day11;

impl crate::Day for Day11 {
    fn run(input: String) -> crate::DayResult {
        let edges = input
            .lines()
            .map(|line| {
                let (src, rest) = line.split_once(":").unwrap();
                let dsts = rest.trim().split(" ").collect::<Vec<&str>>();

                dsts.iter()
                    .map(|dst| (src, *dst))
                    .collect::<Vec<(&str, &str)>>()
            })
            .fold(Vec::new(), |mut acc, mut x| {
                acc.append(&mut x);
                acc
            });

        let graph = DiGraph::from_edges(&edges);

        crate::DayResult {
            part_1: graph.num_paths("you", "out"),
            part_2: graph.num_paths_visit_vertices("svr", "out"),
        }
    }
}

#[derive(Debug)]
struct DiGraph<'a> {
    vertices: HashMap<&'a str, Vertex<'a>>,
}

#[derive(Debug, Default, Clone)]
struct Vertex<'a> {
    in_degree: u64,
    neighbors: Vec<&'a str>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct DFSEntry<'a> {
    name: &'a str,
    in_degree: u64,
    neighbors: Vec<&'a str>,
    num_paths_through_neither: u64,
    num_paths_through_dac: u64,
    num_paths_through_fft: u64,
    num_paths_through_both: u64,
}

impl<'a> DiGraph<'a> {
    pub fn from_edges(edges: &[(&'a str, &'a str)]) -> Self {
        let mut vertices: HashMap<&str, Vertex<'a>> = HashMap::new();

        for &(src, dst) in edges {
            vertices.entry(src).or_default().neighbors.push(dst);
            vertices.entry(dst).or_default().in_degree += 1;
        }

        DiGraph { vertices }
    }

    pub fn num_paths(&self, src_vertex: &'a str, dst_vertex: &'a str) -> u64 {
        let mut num_paths = 0;
        let mut to_visit = self.vertices.get(src_vertex).unwrap().neighbors.clone();

        while let Some(current_vertex) = to_visit.pop() {
            if current_vertex == dst_vertex {
                num_paths += 1;
                continue;
            }

            for next_vertex in self.vertices.get(current_vertex).unwrap().neighbors.iter() {
                to_visit.push(*next_vertex);
            }
        }

        num_paths
    }

    pub fn num_paths_visit_vertices(&self, src_vertex: &'a str, dst_vertex: &'a str) -> u64 {
        let mut vertices_to_process = vec![DFSEntry {
            name: src_vertex,
            in_degree: self.vertices.get(src_vertex).unwrap().in_degree,
            neighbors: self.vertices.get(src_vertex).unwrap().neighbors.clone(),
            num_paths_through_neither: 1,
            num_paths_through_dac: 0,
            num_paths_through_fft: 0,
            num_paths_through_both: 0,
        }];

        while let Some(vertex) = vertices_to_process
            .iter()
            .min_by(|a, b| a.in_degree.cmp(&b.in_degree))
            .cloned()
        {
            if vertex.name == dst_vertex {
                return vertex.num_paths_through_both;
            }

            for neighbor in &vertex.neighbors {
                let entry = vertices_to_process
                    .iter_mut()
                    .find(|entry| entry.name == *neighbor);
                let entry = if let Some(entry) = entry {
                    entry
                } else {
                    vertices_to_process.push(DFSEntry {
                        name: neighbor,
                        in_degree: self.vertices.get(neighbor).unwrap().in_degree,
                        neighbors: self.vertices.get(neighbor).unwrap().neighbors.clone(),
                        num_paths_through_neither: 0,
                        num_paths_through_dac: 0,
                        num_paths_through_fft: 0,
                        num_paths_through_both: 0,
                    });
                    vertices_to_process.last_mut().unwrap()
                };

                if *neighbor == "dac" {
                    entry.num_paths_through_dac +=
                        vertex.num_paths_through_neither + vertex.num_paths_through_dac;
                    entry.num_paths_through_both +=
                        vertex.num_paths_through_fft + vertex.num_paths_through_both;
                } else if *neighbor == "fft" {
                    entry.num_paths_through_fft +=
                        vertex.num_paths_through_neither + vertex.num_paths_through_fft;
                    entry.num_paths_through_both +=
                        vertex.num_paths_through_dac + vertex.num_paths_through_both;
                } else {
                    entry.num_paths_through_neither += vertex.num_paths_through_neither;
                    entry.num_paths_through_dac += vertex.num_paths_through_dac;
                    entry.num_paths_through_fft += vertex.num_paths_through_fft;
                    entry.num_paths_through_both += vertex.num_paths_through_both;
                }

                entry.in_degree -= 1;
            }

            let index = vertices_to_process
                .iter()
                .position(|v| v.name == vertex.name)
                .unwrap();
            vertices_to_process.remove(index);
        }

        panic!("Shouldn't reach here")
    }
}
