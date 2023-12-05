use std::fs::read_to_string;
use std::io::{self, Write};

use itertools::Itertools;
use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct Mapping {
    source: String,
    destination: String,
    
    ranges: Vec<Range>,
}

#[derive(Debug)]
pub struct Range {
    source_start: usize,
    destination_start: usize,
    length: usize,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct BaseRange {
    start: usize,
    end: usize,
}

impl BaseRange {
    fn new(start: usize, end: usize) -> Self {
        Self {
            start, end
        }
    } 
}

impl From<&str> for Range {
    fn from(line: &str) -> Self {
        let values: Vec<_> = line.split(" ").map(|x| x.parse().unwrap()).collect();
        Self {
            source_start: values[1],
            destination_start: values[0],
            length: values[2],
        }
    }
}

impl Range {
    fn forward_convert(&self, value: usize) -> Option<usize> {
        if value >= self.source_start && value < self.source_start + self.length {
            Some(self.destination_start + (value - self.source_start))
        } else {
            None
        }
    }

    fn range_convert(&self, range: BaseRange) -> HashSet<BaseRange> {
        let mut ranges = HashSet::new();
        
        let a = range.start;
        let b = self.source_start;
        let c = self.source_start + self.length - 1;
        let d = range.end;

        let inter_a = a.max(b);
        let inter_b = c.min(d);

        // mapped range
        if inter_a != inter_b {
            ranges.insert(BaseRange::new(inter_a, inter_b));
        }

        // First not mapped range 
        if a.min(b) != inter_a {
            ranges.insert(BaseRange::new(a.min(b), inter_a));

        }

        // Second not mapped range 
        if c.max(d) != inter_b {
            ranges.insert(BaseRange::new(inter_b, c.max(d)));

        }

        ranges
    }
}

impl From<&str> for Mapping {
    fn from(value: &str) -> Self {
        let mut lines = value.lines();

        // Get the destination names
        let (source, destination) = lines
            .next().unwrap()
            .strip_suffix(" map:").unwrap()
            .split_once("-to-").unwrap();

        // Get the ranges
        let ranges = lines.map(|line| Range::from(line)).collect_vec();

        Mapping { source: source.to_string(), destination: destination.to_string(), ranges }
    }
}

impl Mapping {
    fn forward_convert(&self, value: usize) -> usize {
        match self.ranges.iter().filter_map(|x| x.forward_convert(value)).exactly_one() {
            Ok(converted_val) => converted_val,
            Err(..) => value
        }
    }

    fn range_convert(&self, range: BaseRange) -> HashSet<BaseRange> {
        let mut ranges = HashSet::new();
        for convert_range in &self.ranges {
            ranges.extend((*convert_range).range_convert(range.clone()));
        }
        ranges
    }
}

pub struct Solution {
    lines: Vec<String>,
    content: String,
    mappings: HashMap<String, Mapping>,
    memo: HashMap<(String, usize), usize>, // Maps a state and value in the state into the location value
    seeds: Vec<usize>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();

        let content = read_to_string("inputs/day5.txt").unwrap();
        for line in content.lines() {
            lines.push(line.to_string());
        }

        // Data parsing
        let blocks = content.split("\n\n").collect_vec();

        let seeds = blocks[0].strip_prefix("seeds: ").unwrap().split(" ").map(|x| x.parse::<usize>().unwrap()).collect_vec();
        let mut mappings = HashMap::new();
        
        for mapping_block in &blocks[1..] {
            let mapping = Mapping::from(*mapping_block);
            mappings.insert(mapping.source.clone(), mapping);
        }

        Self {
            lines,
            content,
            mappings,
            seeds,
            memo: HashMap::new(),
        }
    }



    fn part1(&mut self) -> usize{
        // Solving
        let mut res = usize::MAX;
        for seed in &self.seeds {
            let mut value = *seed;
            let mut source = "seed";
            
            while let Some(mapping) = self.mappings.get(source) {
                value = mapping.forward_convert(value);
                source = mapping.destination.as_str();
            }

            res = res.min(value);
        }
        res
    }

    fn part2(&mut self) -> usize{
        let mut res = usize::MAX;
        for i in 0..self.seeds.len() / 2 {
            let mut source = "seed";
            let mut seed_ranges = HashSet::new();
            seed_ranges.insert(BaseRange::new(self.seeds[2*i], self.seeds[2*i]+self.seeds[2*i+1]-1));

            println!("range: {:?}", seed_ranges);
            
            while let Some(mapping) = self.mappings.get(source) {
                let mut new_ranges = HashSet::new();
                for range in seed_ranges {
                    new_ranges.extend(mapping.range_convert(range));
                }
                seed_ranges = new_ranges;
                source = mapping.destination.as_str();
            }


            let value = seed_ranges.iter().map(|range| range.start).min();
            if value == Some(0) {
                println!("val: {:?}", value);
                // println!("range: {:?}", seed_ranges);
                panic!();
            }
            println!("     -> {:?}", value);
        }
        res
    }

    pub fn solve(&mut self) {
        println!("========= DAY 5 ========");
        print!("Solving part 1: ");
        io::stdout().flush().unwrap();
        println!("{:?}", self.part1());
        
        print!("Solving part 2: ");
        io::stdout().flush().unwrap();
        println!("{:?}\n", self.part2());
    }
}