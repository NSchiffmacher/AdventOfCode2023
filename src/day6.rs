use std::fs::read_to_string;
use std::io::{self, Write};

pub struct Solution {
    lines: Vec<String>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day6.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        Self {
            lines,
        }
    }

    fn part1(&mut self) {

    }

    fn part2(&mut self) {

    }

    pub fn solve(&mut self) {
        println!("========= DAY 6 ========");
        print!("Solving part 1: ");
        io::stdout().flush().unwrap();
        println!("{:?}", self.part1());
        
        print!("Solving part 2: ");
        io::stdout().flush().unwrap();
        println!("{:?}\n", self.part2());
    }
}