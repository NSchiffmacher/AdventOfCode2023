use std::fs::read_to_string;
use std::io::{self, Write};

use itertools::Itertools;

pub struct Solution {
    lines: Vec<String>,
    instructions: Vec<Instruction>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day18.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        Self {
            instructions: lines.iter().map(|line| Instruction::from(line.as_str())).collect(),
            lines,
        }
    }

    fn part1(&mut self) -> isize {
        // Define variables
        let mut inside_area = 0;
        let mut side_area = 0;

        // Using Green's theorem
        let mut x = 0;
        for instruction in &self.instructions {
            let (dx, dy) = instruction.direction.to_delta();
            let n = instruction.distance;
            
            inside_area += n * x * dy + dx * dy * n * (n + 1) / 2;
            side_area += instruction.distance;

            x += dx * n;
        }

        inside_area + side_area / 2 + 1
    }

    fn part2(&mut self) -> isize {
        // Define variables
        let mut inside_area = 0;
        let mut side_area = 0;

        // Using Green's theorem
        let mut x = 0;
        for instruction in &self.instructions {
            let (dx, dy) = instruction.hex_direction.to_delta();
            let n = instruction.hex_distance;
            
            inside_area += n * x * dy + dx * dy * n * (n + 1) / 2;
            side_area += instruction.hex_distance;

            x += dx * n;
        }

        inside_area + side_area / 2 + 1
    }

    pub fn solve(&mut self) {
        println!("========= DAY 18 ========");
        
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

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: isize,
    hex_distance: isize,
    hex_direction: Direction,
}

impl From<&str> for Instruction {
    // Format is R/L/D/U space number space useless
    fn from(line: &str) -> Self {
        let split = line.split(" ");
        let (direction_str, distance_str, rest_str) = split.take(3).collect_tuple().unwrap();
        let direction = match direction_str {
            "R" => Direction::Right,
            "L" => Direction::Left,
            "D" => Direction::Down,
            "U" => Direction::Up,
            _ => unreachable!("Invalid direction"),
        };
        let distance = distance_str.parse::<isize>().unwrap();

        let rest = rest_str.strip_prefix("(#").unwrap().strip_suffix(")").unwrap();
        let n = rest.len();
        let hex_direction = match &rest[n-1..n] {
            "0" => Direction::Right,
            "1" => Direction::Down,
            "2" => Direction::Left,
            "3" => Direction::Up,
            _ => unreachable!("Invalid direction"),
        };

        Self {
            direction,
            distance,
            hex_distance: isize::from_str_radix(&rest[..n-1], 16).unwrap(),
            hex_direction,
        }
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
    fn to_delta(&self) -> (isize, isize) {
        match self {
            Self::Up => (0, -1),
            Self::Right => (1, 0),
            Self::Down => (0, 1),
            Self::Left => (-1, 0),
        }
    }
}