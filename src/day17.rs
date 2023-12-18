use std::cmp::Reverse;
use std::collections::{HashSet, HashMap};
use std::fs::read_to_string;
use std::io::{self, Write};

use priority_queue::PriorityQueue;

use itertools::Itertools;

pub struct Solution {
    lines: Vec<String>,
    map: Vec<Vec<isize>>,
    width: isize,
    height: isize,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day17.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        let mut map = vec![];
        for line in &lines {
            map.push(line.chars().map(|c| c.to_string().parse::<isize>().unwrap()).collect_vec());
        }

        Self {
            width: lines[0].len() as isize,
            height: lines.len() as isize,
            lines,
            map,
        }
    }

    fn part1(&mut self) -> usize {
        let goal = (self.width - 1, self.height - 1);
        let mut res = 0;

        let mut distances: HashMap<((isize, isize), Direction), isize> = HashMap::new();
        let mut visited = HashSet::new();
        let mut queue = PriorityQueue::new();

        distances.insert(((0, 0), Direction::Right), 0);
        queue.push(((0, 0), Direction::Right), Reverse(0));

        while let Some(((current, direction), heat_loss)) = queue.pop() {
            if current == goal {
                res = heat_loss.0 as usize;
                break;
            }
            if !visited.insert((current, direction)) {
                // Déjà vu
                continue;   
            }

            // Get the possible directions
            let directions = if current == (0, 0) {
                vec![Direction::Right, Direction::Down]
            }  else {
                vec![direction.turn_left(), direction.turn_right()]
            };

            for neighbor_direction in directions {
                let delta = neighbor_direction.to_delta();
                let mut cost = *distances.get(&(current, direction)).unwrap();
                for d in 1..4 {
                    let neighbor = (current.0 + d * delta.0, current.1 + d * delta.1);
                    if neighbor.0 < 0 || neighbor.0 >= self.width || neighbor.1 < 0 || neighbor.1 >= self.height {
                        break; // Skip if it goes out of the map
                    }
                    
                    cost += self.map[neighbor.1 as usize][neighbor.0 as usize];
                    if cost < *distances.get(&(neighbor, neighbor_direction)).unwrap_or(&isize::MAX){
                        distances.insert((neighbor, neighbor_direction), cost);
                        queue.push((neighbor, neighbor_direction), Reverse(cost));
                    }
                }
            }
        }

        res
    }

    fn part2(&mut self) -> usize {
        let goal = (self.width - 1, self.height - 1);
        let mut res = 0;

        let mut distances: HashMap<((isize, isize), Direction), isize> = HashMap::new();
        let mut visited = HashSet::new();
        let mut queue = PriorityQueue::new();

        distances.insert(((0, 0), Direction::Right), 0);
        queue.push(((0, 0), Direction::Right), Reverse(0));

        while let Some(((current, direction), heat_loss)) = queue.pop() {
            if current == goal {
                res = heat_loss.0 as usize;
                break;
            }
            if !visited.insert((current, direction)) {
                // Déjà vu
                continue;   
            }

            // Get the possible directions
            let directions = if current == (0, 0) {
                vec![Direction::Right, Direction::Down]
            }  else {
                vec![direction.turn_left(), direction.turn_right()]
            };

            for neighbor_direction in directions {
                let delta = neighbor_direction.to_delta();
                let mut cost = *distances.get(&(current, direction)).unwrap();
                for d in 1..11 {
                    let neighbor = (current.0 + d * delta.0, current.1 + d * delta.1);
                    if neighbor.0 < 0 || neighbor.0 >= self.width || neighbor.1 < 0 || neighbor.1 >= self.height {
                        break; // Skip if it goes out of the map
                    }
                    
                    cost += self.map[neighbor.1 as usize][neighbor.0 as usize];
                    if d >= 4 && cost < *distances.get(&(neighbor, neighbor_direction)).unwrap_or(&isize::MAX){
                        distances.insert((neighbor, neighbor_direction), cost);
                        queue.push((neighbor, neighbor_direction), Reverse(cost));
                    }
                }
            }
        }

        res
    }

    pub fn solve(&mut self) {
        println!("========= DAY 17 ========");
        
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    fn turn_left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Right => Self::Up,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
        }
    }

    fn turn_right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Right => Self::Down,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
        }
    }

    fn to_right(&self, pos: (isize, isize)) -> ((isize, isize), Self) {
        let d = self.turn_right();
        let delta = d.to_delta();
        ((pos.0 + delta.0, pos.1 + delta.1), d)
    }

    fn to_left(&self, pos: (isize, isize)) -> ((isize, isize), Self) {
        let d = self.turn_left();
        let delta = d.to_delta();
        ((pos.0 + delta.0, pos.1 + delta.1), d)
    }

    fn forward(&self, pos: (isize, isize)) -> ((isize, isize), Self) {
        let delta = self.to_delta();
        ((pos.0 + delta.0, pos.1 + delta.1), self.clone())
    }
 
    fn to_delta(&self) -> (isize, isize) {
        match self {
            Self::Up => (0, -1),
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
        }
    }
}