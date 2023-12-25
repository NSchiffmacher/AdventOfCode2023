use std::fs::read_to_string;
use std::io::{self, Write};

use itertools::Itertools;

use std::collections::{HashMap, HashSet, VecDeque};
use rand::prelude::*;

pub struct Solution {
    lines: Vec<String>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day25.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        Self {
            lines,
        }
    }

    fn dfs(&self, start: String, goal: String, graph: &HashMap<String, Vec<String>>) -> Vec<String> {
        let mut queue = VecDeque::new();
        let mut parent = HashMap::new();

        queue.push_back(start.clone());

        while let Some(element) = queue.pop_front() {
            if element == goal {
                break;
            }

            for neigh in graph.get(&element).unwrap() {
                if !parent.contains_key(neigh) {
                    parent.insert(neigh.clone(), element.clone());
                    queue.push_back(neigh.clone());
                }
            }
        }

        // Reconstruct path
        let mut path = vec![];
        let mut current = goal;
        while current != start {
            path.push(current.clone());
            current = parent.get(&current).unwrap().clone();
        }
        path.push(start);

        path
    }

    fn part1(&mut self) -> usize {
        // Parse
        let mut graph = HashMap::new();
        // let mut edges = HashSet::new();

        for line in &self.lines {
            let (a, end) = line.split(": ").collect_tuple().unwrap();
            for b in end.split(" ") {
                let a = a.to_string();
                let b = b.to_string();

                graph.entry(a.clone()).or_insert(Vec::new()).push(b.clone());
                graph.entry(b.clone()).or_insert(Vec::new()).push(a.clone());

                // edges.insert((a.to_string(), b.to_string()));
                // edges.insert((b.to_string(), a.to_string()));
            }
        }

        let nodes = graph.keys().map(|x| x.clone()).collect_vec();

        loop {
            // Take x random pairs of nodes and find the shortest path between them
            // Count the number of times each edge is used
            let mut edges_occurences = HashMap::new();
            for _ in 0..10 {
                // Choose two random nodes
                let a = nodes.choose(&mut rand::thread_rng()).unwrap().clone();
                let b = nodes.choose(&mut rand::thread_rng()).unwrap().clone();
                if a != b {
                    let path = self.dfs(a.clone(), b.clone(), &graph);
                    for edge in path.windows(2) {
                        let mut edge = (edge[0].clone(), edge[1].clone());
                        if !edges_occurences.contains_key(&edge) {
                            edge = (edge.1, edge.0);
                        }

                        *edges_occurences.entry(edge).or_insert(0) += 1;
                    }
                
                }
            }

            // Select the 3 most used edges between 6 different nodes
            let mut used_nodes = HashSet::new();
            let mut edges = vec![];
            for ((a, b), _occurences) in edges_occurences.iter().sorted_by_key(|(_, occurences)| -**occurences) {
                if used_nodes.contains(&a) || used_nodes.contains(&b) {
                    continue;
                }

                edges.push((a.clone(), b.clone()));
                used_nodes.insert(a);
                used_nodes.insert(b);
                if edges.len() >= 3 {
                    break;
                }
            }

            // Remove the edges from the graph
            for (a, b) in &edges {
                graph.get_mut(a).unwrap().retain(|x| x != b);
                graph.get_mut(b).unwrap().retain(|x| x != a);
            }

            // Run dfs on one of the remaining nodes
            let mut queue = VecDeque::new();
            let mut visited = HashSet::new();
            queue.push_back(edges[0].0.clone());

            while let Some(element) = queue.pop_front() {
                for neigh in graph.get(&element).unwrap() {
                    if visited.insert(neigh) {
                        queue.push_back(neigh.clone());
                    }
                }
            }

            let n = visited.len() * (nodes.len()-visited.len());
            if n != 0 {
                break n;
            }
        }
    }

    fn part2(&mut self) -> String {
        "Merry Christmas!".to_string()
    }

    pub fn solve(&mut self) {
        println!("========= DAY 25 ========");
        print!("Solving part 1: ");
        io::stdout().flush().unwrap();

        let start = std::time::Instant::now();
        let part1 = self.part1();
        let part1_time = start.elapsed();
        println!("{:?} (took {:?})", part1, part1_time);

        print!("Solving part 2: ");
        io::stdout().flush().unwrap();
        let start = std::time::Instant::now();
        let part2 = self.part2();
        let part2_time = start.elapsed();
        println!("{:?} (took {:?})", part2, part2_time);
        println!();
    }
}