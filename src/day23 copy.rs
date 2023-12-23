use std::collections::{HashSet, VecDeque};
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
        let start = (1, 0);
        let goal = (self.width - 2, self.height - 1);
        let mut longest_path = 0;
        let mut visited = HashSet::new();

        self.dfs(start, 0, goal, &mut visited, &mut longest_path);

        longest_path
    }

    fn dfs(&self, position: (isize, isize), distance: usize, goal: (isize, isize), visited: &mut HashSet<(isize, isize)>, longest_path: &mut usize) {
        if position == goal {
            if distance > *longest_path {
                *longest_path = distance ;
            }
            return;
        }

        for neighbor in self.neighbors(position) {
            if !visited.contains(&neighbor) {
                visited.insert(neighbor);
                self.dfs(neighbor, distance + 1, goal, visited, longest_path);
                visited.remove(&neighbor);
            }
        }
    }

    fn part2(&mut self) {

    }

    fn neighbors(&self, (x, y): (isize, isize)) -> Vec<(isize, isize)> {
        let directions = match self.map[y as usize][x as usize] {
            '^' => vec![(0, -1)],
            '>' => vec![(1, 0)],
            'v' => vec![(0, 1)],
            '<' => vec![(-1, 0)],
            '.' | 'O' => vec![(0, 1), (0, -1), (1, 0), (-1, 0)],
            _ => panic!("Invalid direction {} at ({}, {})", self.map[y as usize][x as usize], x, y),
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