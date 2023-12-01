use std::fs::read_to_string;
use std::io::{self, Write};

pub struct Solution {
    lines: Vec<String>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day1.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        Self {
            lines,
        }
    }

    fn part1(&mut self) -> u32 {
        let mut total = 0;

        for line in &self.lines {
            let mut first_digit = Option::None;
            let mut last_digit = Option::None;

            for c in line.chars() {
                if c.is_digit(10) {
                    let d = c.to_digit(10).unwrap();
                    if first_digit.is_none() {
                        first_digit = Some(d)
                    }
                    last_digit = Some(d);
                }
            }

            if first_digit.is_none() || last_digit.is_none() {
                panic!("One of them is none on line \"{}\": {:?} {:?}", line, first_digit, last_digit);
            }

            total += first_digit.unwrap() * 10 + last_digit.unwrap();
        }

        total
    }

    fn text_to_digit(&self, line: &str) -> Option<u32> {
        match line {
            line if line.starts_with("one") => Some(1),
            line if line.starts_with("two") => Some(2),
            line if line.starts_with("three") => Some(3),
            line if line.starts_with("four") => Some(4),
            line if line.starts_with("five") => Some(5),
            line if line.starts_with("six") => Some(6),
            line if line.starts_with("seven") => Some(7),
            line if line.starts_with("eight") => Some(8),
            line if line.starts_with("nine") => Some(9),
            _ => None
        }
    }

    fn part2(&mut self) -> u32{
        let mut total = 0;

        for line in &self.lines {
            let mut first_digit = Option::None;
            let mut last_digit = Option::None;

            let mut chars = line.chars();

            for i in 0..line.len() {
                let c = chars.next().unwrap();

                let d = if c.is_digit(10) {
                    c.to_digit(10)
                } else {
                    self.text_to_digit(&line[i..])
                };

                if let Some(d) = d {
                    if first_digit.is_none() {
                        first_digit = Some(d)
                    }
                    last_digit = Some(d);
                }
            }

            if first_digit.is_none() || last_digit.is_none() {
                panic!("One of them is none on line \"{}\": {:?} {:?}", line, first_digit, last_digit);
            }

            total += first_digit.unwrap() * 10 + last_digit.unwrap();
        }

        total
    }

    pub fn solve(&mut self) {
        println!("========= DAY 1 ========");
        print!("Solving part 1: ");
        io::stdout().flush().unwrap();
        println!("{:?}", self.part1());
        
        print!("Solving part 2: ");
        io::stdout().flush().unwrap();
        println!("{:?}\n", self.part2());
    }
}