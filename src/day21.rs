use std::collections::{VecDeque, HashSet, HashMap};
use std::fs::read_to_string;
use std::io::{self, Write};

pub struct Solution {
    lines: Vec<String>,
    map: Vec<Vec<char>>,
    width: isize,
    height: isize,
    starting_position: (isize, isize),
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        let mut starting_position = None;
        let mut map = vec![];
        for (y, line) in read_to_string("inputs/day21.txt").unwrap().lines().enumerate() {
            let mut row = vec![];
            for (x, c) in line.chars().enumerate() {
                row.push(c);
                if c == 'S' {
                    starting_position = Some((x as isize, y as isize));
                }
            }
            map.push(row);
            lines.push(line.to_string());
        }

        Self {
            lines,
            width: map[0].len() as isize,
            height: map.len() as isize,
            map,
            starting_position: starting_position.expect("No starting position found"),
        }
    }

    fn get(&self, x: isize, y: isize) -> char {
        self.map[y.rem_euclid(self.height) as usize][x.rem_euclid(self.width) as usize]
    }
    
    fn part1(&mut self) -> usize {
        let mut queue = VecDeque::new();
        let mut visited = HashSet::new();
        let max_steps = 64;
        
        queue.push_back((0, self.starting_position));
        visited.insert((0, self.starting_position));
        
        while let Some((steps, (x, y))) = queue.pop_front() {
            for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nx = x + dx;
                let ny = y + dy;
                let nsteps = steps + 1;
                if self.get(nx, ny) != '#' && visited.insert((nsteps, (nx, ny))) && nsteps <= max_steps {
                    queue.push_back((nsteps, (nx, ny)));
                }
            }
        }

        visited.iter().filter(|(steps, _)| *steps == max_steps).count()
    }

    fn part2(&mut self) -> usize {
        // Because of the input shape, it follows a second degree polynomial
        // Kinda hard to guess...

        let mut queue = VecDeque::new();
        let mut distances = HashMap::new();
        
        queue.push_back((0, self.starting_position));
        distances.insert(self.starting_position, 0);
        
        let target_steps = 26501365;
        let offset = target_steps % self.height;
        let magical_numbers = vec![offset, offset + self.height, offset + 2 * self.height];
        let mut magical_values = HashMap::new();

        let max_steps = magical_numbers[2];
        
        while let Some((steps, (x, y))) = queue.pop_front() {
            if magical_numbers.contains(&steps) && !magical_values.contains_key(&steps) {
                // Because we're using DFS, the first time we explore a node at the correct step count, all 
                // nodes with a step count <= have been explored
                // we can thus compute the number of tiles we can reach by taking the number of nodes
                // that have the same parity as the given step count
                // (works because if it's not the same parity, we will never be able to reach it
                // and if it has the same parity, but not the right distance, we can keep going back in forth between two tiles)
                magical_values.insert(steps, distances.values().filter(|d| isize::rem_euclid(**d, 2) == steps.rem_euclid(2)).count());
            }
            for (dx, dy) in &[(0, 1), (0, -1), (1, 0), (-1, 0)] {
                let nx = x + dx;
                let ny = y + dy;
                let nsteps = steps + 1;
                if self.get(nx, ny) != '#' && !distances.contains_key(&(nx, ny)) && nsteps <= max_steps {
                    distances.insert((nx, ny), nsteps);
                    queue.push_back((nsteps, (nx, ny)));
                }
            }
        }

        let s0 = magical_values[&magical_numbers[0]];
        let s1 = magical_values[&magical_numbers[1]];
        let s2 = magical_values[&magical_numbers[2]];

        let c = s0;
        let a = (s2 - 2 * s1 + c) / 2;
        let b = s1 - c - a;

        let n = (target_steps / self.height) as usize;

        a * n * n + b * n + c
    }

    pub fn solve(&mut self) {
        println!("========= DAY 21 ========");
        
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