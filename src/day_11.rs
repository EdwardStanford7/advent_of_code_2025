use std::collections::BTreeMap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::sync::atomic::{AtomicU64, Ordering};

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
    vertices: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> DiGraph<'a> {
    pub fn from_edges(edges: &[(&'a str, &'a str)]) -> Self {
        let mut vertices: HashMap<&str, Vec<&'a str>> = HashMap::new();

        for &(src, dst) in edges {
            vertices.entry(src).or_default().push(dst);
            vertices.entry(dst).or_default();
        }

        DiGraph { vertices }
    }

    pub fn num_paths(&self, src_vertex: &'a str, dst_vertex: &'a str) -> u64 {
        let mut num_paths = 0;
        let mut to_visit = vec![src_vertex];

        while let Some(current_vertex) = to_visit.pop() {
            if current_vertex == dst_vertex {
                num_paths += 1;
                continue;
            }

            for &next_vertex in self.vertices.get(current_vertex).unwrap().iter() {
                to_visit.push(next_vertex);
            }
        }

        num_paths
    }

    fn num_paths_visit_vertices(&'a self, src_vertex: &'a str, dst_vertex: &'a str) -> u64 {
        let vertices = self.calculate_in_degrees(&self.get_reachable(src_vertex));

        let mut priority_queue = BTreeMap::new();
        let mut src_entry = SearchEntry::new(
            src_vertex,
            vertices.get(src_vertex).unwrap().1,
            &vertices.get(src_vertex).unwrap().0,
        );
        src_entry.npaths_neither = 1;
        priority_queue.insert(src_vertex, src_entry);

        while let Some(current) = priority_queue.values().min().cloned() {
            if current.name == dst_vertex {
                return current.npaths_both;
            }

            for &neighbor in current.neighbors {
                let entry = priority_queue.entry(neighbor).or_insert(SearchEntry::new(
                    neighbor,
                    vertices.get(neighbor).unwrap().1,
                    &vertices.get(neighbor).unwrap().0,
                ));

                entry.in_degree -= 1;

                if neighbor == "dac" {
                    entry.npaths_dac += current.npaths_neither + current.npaths_dac;
                    entry.npaths_both += current.npaths_fft + current.npaths_both;
                } else if neighbor == "fft" {
                    entry.npaths_fft += current.npaths_neither + current.npaths_fft;
                    entry.npaths_both += current.npaths_dac + current.npaths_both;
                } else {
                    entry.npaths_neither += current.npaths_neither;
                    entry.npaths_dac += current.npaths_dac;
                    entry.npaths_fft += current.npaths_fft;
                    entry.npaths_both += current.npaths_both;
                }
            }

            priority_queue.remove(current.name);
        }

        unreachable!()
    }

    fn get_reachable(&self, src_vertex: &'a str) -> HashSet<&'a str> {
        let mut reachable = HashSet::new();

        let mut to_visit = vec![src_vertex];

        while let Some(current_vertex) = to_visit.pop() {
            reachable.insert(current_vertex);
            for &next_vertex in self.vertices.get(current_vertex).unwrap().iter() {
                if reachable.contains(next_vertex) {
                    continue;
                }
                to_visit.push(next_vertex);
            }
        }

        reachable
    }

    fn calculate_in_degrees(
        &self,
        reachable: &HashSet<&'a str>,
    ) -> HashMap<&'a str, (Vec<&'a str>, u64)> {
        let mut vertices = HashMap::new();

        for &vertex in reachable {
            vertices.entry(vertex).or_insert((Vec::new(), 0));
            for &neighbor in self.vertices.get(vertex).unwrap() {
                vertices.entry(neighbor).or_insert((Vec::new(), 0));
            }
        }

        for &vertex in reachable {
            for &neighbor in self.vertices.get(vertex).unwrap() {
                vertices.get_mut(vertex).unwrap().0.push(neighbor);
                vertices.get_mut(neighbor).unwrap().1 += 1;
            }
        }

        vertices
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct SearchEntry<'a> {
    name: &'a str,
    in_degree: u64,
    order: u64,
    neighbors: &'a [&'a str],
    npaths_neither: u64,
    npaths_dac: u64,
    npaths_fft: u64,
    npaths_both: u64,
}

impl<'a> SearchEntry<'a> {
    fn new(name: &'a str, in_degree: u64, neighbors: &'a [&'a str]) -> Self {
        static ORDER_COUNTER: AtomicU64 = AtomicU64::new(0);
        let order = ORDER_COUNTER.fetch_add(1, Ordering::Relaxed);
        SearchEntry {
            name,
            in_degree,
            order,
            neighbors,
            npaths_neither: 0,
            npaths_dac: 0,
            npaths_fft: 0,
            npaths_both: 0,
        }
    }
}

impl Ord for SearchEntry<'_> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.in_degree
            .cmp(&other.in_degree)
            .then_with(|| self.order.cmp(&other.order))
    }
}

impl PartialOrd for SearchEntry<'_> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
