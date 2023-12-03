use std::fs::read_to_string;
use std::io::{self, Write};

use std::collections::{HashSet, HashMap};

pub struct Solution {
    lines: Vec<String>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day3.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        Self {
            lines,
        }
    }

    fn part1(&mut self) -> u32{
        let mut sum = 0;

        // Check positions valid given symbols
        let mut valid_positions = HashSet::new();
        for (y, line) in self.lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c != '.' && c.is_ascii_punctuation() {
                    for dx in 0..3 {
                        for dy in 0..3 {
                            valid_positions.insert(((x+dx).saturating_sub(1), (y+dy).saturating_sub(1)));
                        }
                    }
                }
            }
        }

        // Read numbers
        for (y, line) in self.lines.iter().enumerate() {
            let mut x = 0;
            let mut chars = line.chars();
            while let Some(c) = chars.next() {
                if c.is_digit(10) {
                    let mut valid_position = valid_positions.contains(&(x, y));
                    let mut current_number = String::from(c);

                    // Start of number
                    while let Some(c) = chars.next() {
                        x += 1;
                        
                        if !c.is_digit(10) {
                            break;
                        }

                        valid_position |= valid_positions.contains(&(x, y));
                        current_number.push(c);
                    }

                    // Complete number
                    if valid_position {
                        sum += current_number.parse::<u32>().unwrap();
                    }
                }
                
                x += 1;
            }
        }

        sum
    }

    fn part2(&mut self) -> u32{
        // Check positions valid given symbols
        let mut gear_positions = HashMap::new();
        let mut gear_ratios: HashMap<(usize, usize), (u32, u32)> = HashMap::new();
        for (y, line) in self.lines.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '*' {
                    for dx in 0..3 {
                        for dy in 0..3 {
                            gear_positions.insert(((x+dx).saturating_sub(1), (y+dy).saturating_sub(1)), (x, y));
                        }
                    }
                    gear_ratios.insert((x, y), (0, 1));
                }
            }
        }

        // Read numbers
        for (y, line) in self.lines.iter().enumerate() {
            let mut x = 0;
            let mut chars = line.chars();
            while let Some(c) = chars.next() {
                if c.is_digit(10) {
                    let mut gear_position = gear_positions.get(&(x, y));
                    let mut current_number = String::from(c);

                    // Start of number
                    while let Some(c) = chars.next() {
                        x += 1;
                        
                        if !c.is_digit(10) {
                            break;
                        }

                        if gear_position.is_none() {
                            gear_position = gear_positions.get(&(x, y));
                        }
                        current_number.push(c);
                    }

                    // Complete number
                    if let Some(gear_position) = gear_position {
                        let (cur_num, cur_ratio) = gear_ratios.get(gear_position).unwrap();
                        gear_ratios.insert(*gear_position, (cur_num+1, cur_ratio * current_number.parse::<u32>().unwrap()));
                    }
                }
                
                x += 1;
            }
        }

        gear_ratios.values().into_iter().filter(|(num, _ratio)| *num == 2).map(|(_num, ratio)| *ratio).sum()
    }

    pub fn solve(&mut self) {
        println!("========= DAY 3 ========");
        print!("Solving part 1: ");
        io::stdout().flush().unwrap();
        println!("{:?}", self.part1());
        
        print!("Solving part 2: ");
        io::stdout().flush().unwrap();
        println!("{:?}\n", self.part2());
    }
}