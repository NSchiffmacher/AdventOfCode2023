use std::collections::{HashSet, VecDeque, HashMap};
use std::fs::read_to_string;
use std::io::{self, Write};

pub struct Solution {
    lines: Vec<String>,
    width: isize,
    height: isize,
    map: Vec<Vec<char>>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        let mut map = Vec::new();
        for line in read_to_string("inputs/day23.txt").unwrap().lines() {
            map.push(line.chars().collect());
            lines.push(line.to_string());
        }

        Self {
            width: lines[0].len() as isize,
            height: lines.len() as isize,
            lines,
            map,
        }
    }

    fn part1(&mut self) -> usize {
        self.solve_part(1)
    }

    fn part2(&mut self) -> usize{
        self.solve_part(2)
    }

    fn dfs(&self, position: (isize, isize), distance: usize, goal: (isize, isize), visited: &mut HashSet<(isize, isize)>, longest_path: &mut usize, graph: &HashMap<(isize, isize), Vec<((isize, isize), usize)>>) {
        if position == goal {
            if distance > *longest_path {
                *longest_path = distance ;
            }
            return;
        }

        for (neighbor, cost) in graph.get(&position).unwrap() {
            if !visited.contains(&neighbor) {
                visited.insert(*neighbor);
                self.dfs(*neighbor, distance + cost, goal, visited, longest_path, graph);
                visited.remove(&neighbor);
            }
        }
    }

    fn solve_part(&self, part: usize) -> usize {
        let start = (1, 0);
        let goal = (self.width - 2, self.height - 1);
        
        // List the intersections
        let mut intersections = HashSet::new();
        for (y, line) in self.map.iter().enumerate() {
            for (x, c) in line.iter().enumerate() {
                if c != &'#' && self.neighbors(part, (x as isize, y as isize)).len() > 2 {
                    intersections.insert((x as isize, y as isize));
                }
            }
        }

        intersections.insert(start);
        intersections.insert(goal);

        // Create the graph (basically DFS from each intersection to the others)
        let mut graph = HashMap::new();
        for intersection in &intersections {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back((*intersection, 0));
            visited.insert(*intersection);

            while let Some((pos, dist)) = queue.pop_front() {
                if intersections.contains(&pos) && pos != *intersection {
                    // We've reached another intersection
                    graph.entry(*intersection).or_insert(Vec::with_capacity(4)).push((pos, dist));
                    continue; // Stop that path
                }

                for neighbor in self.neighbors(part, pos) {
                    if !visited.contains(&neighbor) {
                        visited.insert(neighbor);
                        queue.push_front((neighbor, dist + 1));
                    }
                }
            }
        }

        // Run actual DFS 
        let mut longest_path = 0;
        let mut visited = HashSet::new();

        self.dfs(start, 0, goal, &mut visited, &mut longest_path, &graph);

        longest_path
    }

    fn neighbors(&self, part: usize, (x, y): (isize, isize)) -> Vec<(isize, isize)> {
        let directions = if part == 1 {
            match self.map[y as usize][x as usize] {
            '^' => vec![(0, -1)],
            '>' => vec![(1, 0)],
            'v' => vec![(0, 1)],
            '<' => vec![(-1, 0)],
            '.' | 'O' => vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
            _ => panic!("Invalid direction {} at ({}, {})", self.map[y as usize][x as usize], x, y),
            }
        } else {
            vec![(0, 1), (0, -1), (1, 0), (-1, 0)]
        };

        let mut res = vec![];
        for (dx, dy) in directions {
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0 && nx < self.width && ny >= 0 && ny < self.height && self.map[ny as usize][nx as usize] != '#' {
                res.push((nx, ny));
            }
        }
        res
    }

    pub fn solve(&mut self) {
        println!("========= DAY 23 ========");
        
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