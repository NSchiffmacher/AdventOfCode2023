use std::fs::read_to_string;
use std::io::{self, Write};

use itertools::Itertools;

pub struct Solution {
    lines: Vec<String>,
    values: Vec<Vec<i64>>,
    derivates: Vec<Vec<Vec<i64>>>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day9.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        // Parse the values
        let values = lines
            .iter()
            .map(|line| line  
                                    .split_whitespace()
                                    .map(|v| v.parse::<i64>().unwrap())
                                    .collect_vec())
            .collect_vec();

        
        // Compute the "derivates"
        let mut derivates: Vec<Vec<Vec<i64>>> = vec![];
        for values_list in &values {
            let mut current_derivates = vec![values_list.clone()];
            while current_derivates.last().unwrap().iter().any(|x| *x != 0) {
                let next_order_derivates = current_derivates.last().unwrap()
                    .iter()
                    .zip(current_derivates.last().unwrap().iter().skip(1))
                    .map(|(a, b)| *b - *a)
                    .collect_vec();
                current_derivates.push(next_order_derivates);
            }
            derivates.push(current_derivates);
        }

        Self {
            lines,
            values,
            derivates,
        }
    }

    fn part1(&mut self) -> i64 {
        self
            .derivates
            .iter()
            .map(|derivate| self.extrapolate_end(derivate))
            .sum()
    }

    fn part2(&mut self) -> i64 {
        self
            .derivates
            .iter()
            .map(|derivate| self.extrapolate_start(derivate))
            .sum()
    }

    fn extrapolate_end(&self, derivates: &Vec<Vec<i64>>) -> i64 {
        // Intregrate the result
        let mut current_result = 0;
        for derivate in derivates.iter().rev() {
            current_result += derivate.last().unwrap();
        }
        current_result
    }

    fn extrapolate_start(&self, derivates: &Vec<Vec<i64>>) -> i64 {
        // Intregrate the result
        let mut current_result = 0;
        for derivate in derivates.iter().rev() {
            current_result = derivate.first().unwrap() - current_result;
        }
        current_result
    }

    pub fn solve(&mut self) {
        println!("========= DAY 9 ========");
        
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