use std::fs::read_to_string;
use std::io::{self, Write};

use itertools::Itertools;
use std::collections::HashMap;

use threadpool::ThreadPool;
use std::sync::{mpsc, Arc, RwLock};

#[derive(Debug, Clone)]
pub struct SeedRange {
    start: usize,
    end: usize,
}

#[derive(Debug, Clone)]
pub struct Mapping {
    pub source: String,
    pub destination: String,

    pub ranges: Vec<Range>,
}

#[derive(Debug, Clone)]
pub struct Range {
    source_start: usize,
    destination_start: usize,
    length: usize,
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
}

pub struct Solution {
    lines: Vec<String>,
    content: String,
    mappings: HashMap<String, Mapping>,
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
        }
    }

    fn part1(&mut self) -> usize{
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
        // Compute the different ranges
        let threads = 256;
        let mut ranges = vec![];

        for i in 0..self.seeds.len() / 2 {
            ranges.push(SeedRange{ start: self.seeds[2*i], end: self.seeds[2*i]+self.seeds[2*i+1]});
        }

        while ranges.len() < threads {
            let index = ranges.iter().position_max_by_key(|x| x.end - x.start).unwrap();
            let (start, end) = (ranges[index].start, ranges[index].end);
            let half = (start + end) / 2;
            ranges.remove(index);
            ranges.push(SeedRange { start, end: half });
            ranges.push(SeedRange { start: half+1, end });
        }

        // Multithread the computation
        let pool = ThreadPool::new(threads);
        // let mappings = Arc::new(RwLock::new(self.mappings.clone()));
        let (tx, rx) = mpsc::channel();

        for range in ranges {
            let mappings = self.mappings.clone();
            let tx = tx.clone();
            pool.execute(move || {
                let mut res = usize::MAX;
                for seed in range.start..range.end {
                    let mut value = seed;
                    let mut source = "seed";
                    
                    while let Some(mapping) = mappings.get(source) {
                        value = mapping.forward_convert(value);
                        source = mapping.destination.as_str();
                    }
        
                    res = res.min(value);
                }

                let _ = tx.send(res);
            })
        }

        rx.iter().take(threads).min().unwrap()
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