use std::fs::read_to_string;
use std::io::{self, Write};

use itertools::Itertools;

pub struct Solution {
    patterns: Vec<Vec<Vec<char>>>,
    patterns_transposed: Vec<Vec<Vec<char>>>,
}

impl Solution {
    pub fn init() -> Self {
        let content = read_to_string("inputs/day13.txt").unwrap();
        let blocks = content.split("\n\n").collect_vec();
        let mut patterns = vec![];
        let mut patterns_transposed = vec![];
        for block in blocks {
            let regular = block.lines().map(|line| line.chars().collect_vec()).collect_vec();
            let mut transposed = vec![vec![]; regular[0].len()];

            for (_i, line) in block.lines().map(|s| s.to_string()).collect_vec().iter().enumerate() {
                for (j, c) in line.chars().enumerate() {
                    transposed[j].push(c);
                }
            }
            
            patterns.push(regular);
            patterns_transposed.push(transposed);
        }

        Self {
            patterns,
            patterns_transposed,
        }
    }

    fn part1(&mut self) -> usize{
        let mut res = 0;
        for i in 0..self.patterns.len() {
            res += self.analyze_rows(&self.patterns[i], None) * 100 
                 + self.analyze_rows(&self.patterns_transposed[i], None)
        }
        res
    }

    fn part2(&mut self) -> usize {
        let mut res = 0;
        for i in 0..self.patterns.len() {
            let base = (self.analyze_rows(&self.patterns[i], None),self.analyze_rows(&self.patterns_transposed[i], None));
            let ignore_a = if base.0 != 0 { Some(base.0 as isize) } else { None };
            let ignore_b = if base.1 != 0 { Some(base.1 as isize) } else { None };
            'main: for y in 0..self.patterns[i].len() {
                for x in 0..self.patterns[i][0].len() {
                    self.patterns[i][y][x] = match self.patterns[i][y][x] {
                        '#' => '.',
                        '.' => '#',
                        _ => panic!("Invalid character"),
                    };
                    self.patterns_transposed[i][x][y] = match self.patterns_transposed[i][x][y] {
                        '#' => '.',
                        '.' => '#',
                        _ => panic!("Invalid character"),
                    };

                    let row = self.analyze_rows(&self.patterns[i], ignore_a);
                    let column = self.analyze_rows(&self.patterns_transposed[i], ignore_b);
                    if row * 100 + column != 0 {
                        res += row * 100 + column; 
                        break 'main;
                    }

                    self.patterns[i][y][x] = match self.patterns[i][y][x] {
                        '#' => '.',
                        '.' => '#',
                        _ => panic!("Invalid character"),
                    };
                    self.patterns_transposed[i][x][y] = match self.patterns_transposed[i][x][y] {
                        '#' => '.',
                        '.' => '#',
                        _ => panic!("Invalid character"),
                    };
                }
            }
        }
        res
    }

    fn try_possibility(&self, lines: &Vec<Vec<char>>, start: usize, end: usize) -> bool {
        let mut i = start;
        let mut j = end;
        while i < j {
            if lines[i] != lines[j] {
                return false;
            }
            i += 1;
            j -= 1;
        }
        i != j
    }

    fn analyze_rows(&self, lines: &Vec<Vec<char>>, ignore: Option<isize>) -> usize {
        let ignore = ignore.unwrap_or(-1);

        // Start from the start 
        let n = lines.len();
        for possibility in (1..n).rev() {
            if (possibility / 2 + 1) as isize != ignore && self.try_possibility(lines, 0, possibility) {
                return possibility / 2 + 1;
            }
        }
        
        // Start from the end 
        let n = lines.len();
        for possibility in 0..n-1 {
            if ((possibility+n-1) / 2 + 1) as isize != ignore && self.try_possibility(lines, possibility, n-1) {
                return (possibility+n-1) / 2 + 1;
            }
        }

        0
    }

    pub fn solve(&mut self) {
        println!("========= DAY 13 ========");
        
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

#[derive(Debug, PartialEq, Eq)]
enum Reflection {
    Vertical(usize),
    Horizontal(usize),
}