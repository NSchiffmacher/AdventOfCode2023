use std::fs::read_to_string;
use std::io::{self, Write};

use itertools::Itertools;
use std::collections::HashSet;

pub struct Solution {
    lines: Vec<String>,
    map: Vec<Vec<char>>,
    width: isize,
    height: isize,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day16.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        let mut map = vec![];
        for line in &lines {
            map.push(line.chars().collect_vec());
        }

        Self {
            width: lines[0].len() as isize,
            height: lines.len() as isize,
            lines,
            map,
        }
    }

    fn update_bean(&self, beam: Beam, cache: &mut HashSet<((isize, isize), (isize, isize))>) -> Vec<Beam> {
        let cell = self.map[beam.y as usize][beam.x as usize];
        let beams = match cell {
            '.' => vec![beam.forward()],
            '\\' => vec![Beam::new(beam.x+beam.dy, beam.y+beam.dx, beam.dy, beam.dx)],
            // '\\' if beam.dx == 0 && beam.dy == 1 => vec![Beam::new(beam.x, beam.y, 1, 0)],
            // '\\' if beam.dx == 0 && beam.dy == -1 => vec![Beam::new(beam.x, beam.y, -1, 0)],
            // '\\' if beam.dx == 1 && beam.dy == 0 => vec![Beam::new(beam.x, beam.y, 0, 1)],
            // '\\' if beam.dx == -1 && beam.dy == 0 => vec![Beam::new(beam.x, beam.y, 0, -1)],
            '/'=> vec![Beam::new(beam.x-beam.dy, beam.y-beam.dx, -beam.dy, -beam.dx)],
            // '/' if beam.dx == 0 && beam.dy == 1 => vec![Beam::new(beam.x, beam.y, -1, 0)],
            // '/' if beam.dx == 0 && beam.dy == -1 => vec![Beam::new(beam.x, beam.y, 1, 0)],
            // '/' if beam.dx == 1 && beam.dy == 0 => vec![Beam::new(beam.x, beam.y, 0, -1)],
            // '/' if beam.dx == -1 && beam.dy == 0 => vec![Beam::new(beam.x, beam.y, 0, 1)],
            '|' if beam.dx == 0 => vec![beam.forward()],
            '-' if beam.dy == 0 => vec![beam.forward()],
            '|' => vec![Beam::new(beam.x, beam.y+1, 0, 1), Beam::new(beam.x, beam.y-1, 0, -1)],
            '-' => vec![Beam::new(beam.x+1, beam.y, 1, 0), Beam::new(beam.x-1, beam.y, -1, 0)],
            _ => panic!("Unknown cell: {}", cell),
        };
        beams.into_iter().filter(|beam| beam.is_valid(self.width, self.height) && cache.insert(beam.as_cache())).collect_vec()
    }

    fn number_energized_from_start(&self, beam: Beam) -> usize {
        let mut beams = vec![beam];
        let mut cache = HashSet::new();

        while !beams.is_empty() {
            let mut new_beams = vec![];
            for beam in beams {
                new_beams.append(&mut self.update_bean(beam, &mut cache));
            }
            beams = new_beams;
        }

        cache.into_iter().map(|(pos, _)| pos).unique().count()
    }

    fn part1(&mut self) -> usize{
        self.number_energized_from_start(Beam::new(0, 0, 1, 0))
    }

    fn part2(&mut self) -> usize {
        let mut best = 0;

        for x in 0..self.width {
            best = best.max(self.number_energized_from_start(Beam::new(x, 0, 0, 1)));
            best = best.max(self.number_energized_from_start(Beam::new(x, self.height-1, 0, -1)));
        }

        for y in 0..self.height {
            best = best.max(self.number_energized_from_start(Beam::new(0, y, 1, 0)));
            best = best.max(self.number_energized_from_start(Beam::new(self.width-1, y, -1, 0)));
        }

        best
    }

    pub fn solve(&mut self) {
        println!("========= DAY 16 ========");
        
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Beam {
    x: isize,
    y: isize,
    dx: isize,
    dy: isize,
}

impl Beam {
    fn new(x: isize, y: isize, dx: isize, dy: isize) -> Self {
        Self {
            x,
            y,
            dx,
            dy,
        }
    }

    fn forward(&self) -> Self {
        Self {
            x: self.x + self.dx,
            y: self.y + self.dy,
            dx: self.dx,
            dy: self.dy,
        }
    }

    fn is_valid(&self, width: isize, height: isize) -> bool {
        self.x >= 0 && self.x < width && self.y >= 0 && self.y < height
    }

    fn as_cache(&self) -> ((isize, isize), (isize, isize)) {
        ((self.x, self.y), (self.dx, self.dy))
    }
}