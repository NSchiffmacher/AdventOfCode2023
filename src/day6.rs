use std::fs::read_to_string;
use std::io::{self, Write};

use itertools::Itertools;

pub struct Solution {
    lines: Vec<String>,
    times_distances: Vec<(i64, i64)>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day6.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        let a = lines[0].strip_prefix("Time:").unwrap().trim();
        let b = lines[1].strip_prefix("Distance:").unwrap().trim();

        Self {
            times_distances: a.split(" ").filter_map(|x| x.parse::<i64>().ok()).zip(b.split(" ").filter_map(|x| x.parse::<i64>().ok())).collect_vec(),
            lines,
        }
    }

    fn find_beaten_records(&self, total_time: i64, record_distance: i64) -> i64 {
        let delta_sqr = ((total_time.pow(2) - 4 * record_distance) as f64).sqrt();
        let start = ((total_time as f64 - delta_sqr) / 2.).ceil() as i64;
        let end = ((total_time as f64 + delta_sqr) / 2.).floor() as i64;
        
        end - start + 1
    }

    fn part1(&mut self) -> i64 {
        let mut res: i64 = 1;
        for (total_time, record_distance) in &self.times_distances {
            res *= self.find_beaten_records(*total_time, *record_distance);
        }
        res
    }

    fn part2(&mut self) -> i64{
        let time: i64 = self.lines[0].strip_prefix("Time:").unwrap().replace(" ", "").parse().unwrap();
        let record_distance: i64 = self.lines[1].strip_prefix("Distance:").unwrap().replace(" ", "").parse().unwrap();
        
        self.find_beaten_records(time, record_distance)
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