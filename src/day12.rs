use std::fs::read_to_string;
use std::io::{self, Write};

use itertools::Itertools;
use std::collections::HashMap;

pub struct Solution {
    lines: Vec<String>,
    entries: Vec<Entry>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day12.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        let mut entries = Vec::with_capacity(lines.len());
        for line in &lines { 
            let (map, pattern) = line.split_once(" ").unwrap();
            let map = map.chars().map(|c| {
                match c {
                    '?' => Tile::Unknown,
                    '.' => Tile::Operational,
                    '#' => Tile::Damaged,
                    _ => panic!("Invalid character in map"),
                }
            }).collect_vec();
            let pattern = pattern.split(",").map(|s| s.parse::<usize>().unwrap()).collect_vec();
            entries.push(Entry::new(map, pattern));            
        }

        Self {
            lines,
            entries,
        }
    }

    fn part1(&mut self) -> usize {
        let mut res = 0;

        for entry in &self.entries {
            res += self.solve_entry(&entry.map, &entry.pattern);
        }

        res
    }

    fn part2(&mut self) -> usize {
        let mut res = 0;

        for entry in &self.entries {
            // Extend the input
            // BEURK
            let mut tmp = entry.map.clone();
            tmp.push(Tile::Unknown);
            let mut map = vec![tmp; 4].iter().flatten().map(|x| *x).collect_vec();
            map.extend(entry.map.clone());
            let pattern = vec![entry.pattern.clone(); 5].iter().flatten().map(|x| *x).collect_vec();
            
            res += self.solve_entry(&map, &pattern);
        }

        res

    }

    fn solve_entry_rec(&self, map: &Vec<Tile>, pattern: &Vec<usize>, num_hash: usize, index: usize, pattern_index: usize, cache: &mut HashMap<(usize, usize, usize), usize>) -> usize {
        if let Some(value) = cache.get(&(num_hash, index, pattern_index)) {
            return *value;
        }

        // We reached the end
        if index == map.len() {
            // We are not in a pattern, and have no more pattern left => This "path" is valid => 1
            if num_hash == 0 && pattern_index == pattern.len() {
                return 1;
            }

            // We are at the last pattern, and we have the correct number of hashes => This "path" is valid => 1
            if pattern_index == pattern.len() - 1 && num_hash == pattern[pattern_index] {
                return 1;
            }

            // Otherwise, we have reached the end but somethings wrong => This "path" is invalid => 0 (possibilities)
            return 0;
        }

        let mut total = 0;
        let curr_tile = map[index];
        // If the current tile is damaged or unknown (in this case we consider it damaged), we continue the pattern
        if curr_tile == Tile::Damaged || curr_tile == Tile::Unknown {
            total += self.solve_entry_rec(map, pattern, num_hash + 1, index + 1, pattern_index, cache);
        }

        // If the current tile is operational or unknown (in this case we consider it operational), we "break" the current pattern
        if curr_tile == Tile::Operational || curr_tile == Tile::Unknown {
            // We are currently not in a pattern, just move forward
            if num_hash == 0 {
                total += self.solve_entry_rec(map, pattern, 0, index + 1, pattern_index, cache);
            } else if pattern_index < pattern.len() && num_hash == pattern[pattern_index] {
                // We successfully end a pattern
                total += self.solve_entry_rec(map, pattern, 0, index + 1, pattern_index + 1, cache);
            }
        }

        // Add to the cache
        cache.insert((num_hash, index, pattern_index), total);

        total
    }

    fn solve_entry(&self, map: &Vec<Tile>, pattern: &Vec<usize>) -> usize {
        let mut cache: HashMap<(usize, usize, usize), usize> = HashMap::new();
        self.solve_entry_rec(map, pattern, 0, 0, 0, &mut cache)
    }

    pub fn solve(&mut self) {
        println!("========= DAY 12 ========");
        print!("Solving part 1: ");
        io::stdout().flush().unwrap();
        println!("{:?}", self.part1());
        
        print!("Solving part 2: ");
        io::stdout().flush().unwrap();
        println!("{:?}\n", self.part2());
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Operational,
    Damaged,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Entry {
    map: Vec<Tile>,
    pattern: Vec<usize>,
}

impl Entry {
    fn new(map: Vec<Tile>, pattern: Vec<usize>) -> Self {
        Self {
            map,
            pattern,
        }
    }
}