use std::fs::read_to_string;
use std::io::{self, Write};

use std::collections::{HashSet, HashMap};

pub struct Solution {
    lines: Vec<String>,
    empty_rows: HashSet<usize>,
    empty_cols: HashSet<usize>,
    galaxies: Vec<Galaxy>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day11.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        let mut empty_rows = HashSet::from_iter(0..lines.len());
        let mut empty_cols = HashSet::from_iter(0..lines[0].len());
        let mut galaxies = Vec::new();
        for (i, line) in lines.iter().enumerate() {
            for (j, cell) in line.chars().enumerate() {
                if cell == '#' {
                    galaxies.push(Galaxy::new(j, i));
                    empty_rows.remove(&i);
                    empty_cols.remove(&j);
                }
            }
        }

        Self {
            lines,
            empty_rows,
            empty_cols,
            galaxies,
        }
    }

    fn compute_result(&self, expansion_factor: usize) -> usize {
        let mut galaxies = self.galaxies.clone();

        // Expand the universe
        for empty_col in &self.empty_cols {
            for galaxy in &mut galaxies {
                if galaxy.original_x > *empty_col {
                    galaxy.x += expansion_factor;
                }
            }
        }
        
        for empty_row in &self.empty_rows {
            for galaxy in &mut galaxies {
                if galaxy.original_y > *empty_row {
                    galaxy.y += expansion_factor;
                }
            }
        }

        // Get the distance between every pair
        let mut distances: HashMap<(usize, usize), usize> = HashMap::new();
        for i in 0..galaxies.len(){
            for j in i+1..galaxies.len(){
                let galaxy_a = &galaxies[i];
                let galaxy_b = &galaxies[j];
                let distance = (galaxy_a.x as isize - galaxy_b.x as isize).abs() as usize
                    + (galaxy_a.y as isize - galaxy_b.y as isize).abs() as usize;

                let d = distances.entry((i, j)).or_insert(distance);
                *d = distance.min(*d);
            }
        }
        
        let mut res = 0;
        for i in 0..galaxies.len(){
            for j in i+1..galaxies.len(){
                res += distances.get(&(i, j)).unwrap();
            }
        }
        res
    }

    fn part1(&mut self) -> usize {
        self.compute_result(1)
    }

    fn part2(&mut self) -> usize {
        self.compute_result(999_999)
    }

    pub fn solve(&mut self) {
        println!("========= DAY 11 ========");
        print!("Solving part 1: ");
        io::stdout().flush().unwrap();
        println!("{:?}", self.part1());
        
        print!("Solving part 2: ");
        io::stdout().flush().unwrap();
        println!("{:?}\n", self.part2());
    }
}


#[derive(Debug, PartialEq, Eq)]
enum Cell {
    Galaxy,
    Empty,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Galaxy {
    original_x: usize,
    original_y: usize,
    x: usize,
    y: usize,
}

impl Galaxy {
    fn new(original_x: usize, original_y: usize) -> Self {
        Self {
            original_x,
            original_y,
            x: original_x,
            y: original_y,
        }
    }
}