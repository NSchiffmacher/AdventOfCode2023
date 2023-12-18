use std::fs::read_to_string;
use std::io::{self, Write};

use itertools::Itertools;

pub struct Solution {
    lines: Vec<String>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day15.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        Self {
            lines,
        }
    }

    fn hash(&self, string: &str) -> u32 {
        let mut current = 0;

        for c in string.chars() {
            let ascii = c as u32;
            current = ((current + ascii) * 17) % 256;
        }

        current
    }

    fn part1(&mut self) -> u32 {
        let mut sum = 0;
        for chunk in self.lines[0].split(",") {
            sum += self.hash(chunk);
        }
        sum
    }

    fn part2(&mut self) -> usize {
        // Box elle a un label mais aussi un num entre 0 et 255 ? 
        let mut boxes: Vec<Vec<(String, u32)>> = vec![vec![]; 256];

        for chunk in self.lines[0].split(",") {
            if chunk.ends_with("-") {
                let label = &chunk[..chunk.len()-1];
                let hash = self.hash(&label.to_string()) as usize;

                boxes[hash] = boxes[hash].iter().cloned().filter(|(l, _focal)| *l != label).collect_vec();
            } else {
                let (label, focal_length) = chunk.split_once("=").unwrap();
                let focal_length = focal_length.parse::<u32>().unwrap();

                let hash = self.hash(label) as usize;
                let mut found = false;

                for (l, old_focal_length) in &mut boxes[hash] {
                    if l == label {
                        *old_focal_length = focal_length;
                        found = true;
                        break;
                    }
                }
                if !found {
                    boxes[hash].push((label.to_string(), focal_length));
                }
            }
        }

        let mut res = 0;
        for box_index in 0..256 {
            for (slot_index, (_label, focal_length)) in boxes[box_index].iter().enumerate() {
                res += (box_index + 1) * (slot_index + 1) * (*focal_length as usize);
            }
        }
        res
    }

    pub fn solve(&mut self) {
        println!("========= DAY 15 ========");
        
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