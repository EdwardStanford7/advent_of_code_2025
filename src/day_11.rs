use std::collections::HashMap;

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

        // dbg!(&graph);

        crate::DayResult {
            part_1: graph.count_paths("you", "out"),
            part_2: 0,
        }
    }
}

#[derive(Debug)]
struct DiGraph<'a> {
    edges: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> DiGraph<'a> {
    pub fn from_edges(edges: &[(&'a str, &'a str)]) -> Self {
        let mut graph = DiGraph {
            edges: HashMap::new(),
        };

        for &(src, dst) in edges {
            graph.edges.entry(src).or_default().push(dst);
        }

        graph
    }

    pub fn count_paths(&self, src_vertex: &'a str, dst_vertex: &'a str) -> u64 {
        let mut num_paths = 0;
        let mut to_visit = self.edges.get(src_vertex).unwrap().clone();

        while let Some(current_vertex) = to_visit.pop() {
            if current_vertex == dst_vertex {
                num_paths += 1;
                continue;
            }

            for next_vertex in self.edges.get(current_vertex).unwrap() {
                to_visit.push(*next_vertex);
            }
        }

        num_paths
    }
}
