use std::fs::read_to_string;
use std::io::{self, Write};

use itertools::Itertools;
use bimap::BiMap;

pub struct Solution {
    lines: Vec<String>,
    map: Vec<Vec<char>>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day14.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        let map = lines.iter().map(|line| line.chars().collect_vec()).collect_vec();

        Self {
            map,
            lines,
        }
    }

    fn normalize(val: isize) -> isize {
        match val {
            val if val < 0 => -1,
            val if val > 0 => 1,
            _ => 0,
        }
    }

    fn apply_gravity(&self, map: &mut Vec<Vec<char>>, gravity: (isize, isize)) {
        // for example, if gravity is (0, -1), we want to move all the rocks up
        // so start from the top, and move down

        let (gx, gy) = gravity;

        let width = map[0].len() as isize;
        let height = map.len() as isize;
        
        let (mut x, mut y, end_x, end_y) = match gravity {
            (0, -1) => (0, 0, width, 0),
            (0, 1) => (0, height-1, width, height-1),
            (-1, 0) => (0, 0, 0, height),
            (1, 0) => (width-1, 0, width-1, height),
            _ => unreachable!("gravity can only be (0, -1), (0, 1), (-1, 0), or (1, 0)"),
        };

        let dx = Self::normalize(end_x - x);
        let dy = Self::normalize(end_y - y);
        let offset = if end_x == x {
            width
        } else {
            height
        };

        while x != end_x || y != end_y {
            // work on the given row/col
            let mut placing_pos = (-1, -1);
            for i in 0..offset {
                let cell = map[(y - gy * i) as usize][(x - gx * i) as usize];
                if cell == 'O' {
                    // Two possibilities, we can place it somewhere or not
                    if placing_pos != (-1, -1) {
                        // We can place it there, so move it
                        map[(y - gy * i) as usize][(x - gx * i) as usize] = '.';
                        map[placing_pos.1 as usize][placing_pos.0 as usize] = 'O';

                        // and set the placing pos to the next one
                        placing_pos = (placing_pos.0 - gx, placing_pos.1 - gy);
                    } 
                }
                else if cell == '#' {
                    // It's a rock, so we need to reset the placing_pos
                    placing_pos = (-1, -1);
                } else if cell == '.' {
                    // if the placing_pos is not set, set it, otherwise leave it
                    if placing_pos == (-1, -1) {
                        placing_pos = (x - gx * i, y - gy * i);
                    }
                } else {
                    unreachable!("cell can only be 'O', '#', or '.'");
                }
            }
            
            x += dx;
            y += dy;
        }
    }

    fn apply_cycle(&self, map: &mut Vec<Vec<char>>) {
        for gravity in [(0, -1), (-1, 0), (0, 1), (1, 0)] {
            self.apply_gravity(map, gravity);
        }
    }

    fn calculate_load(&self, map: &Vec<Vec<char>>) -> usize {
        // let width = map[0].len();
        let height = map.len();

        let mut load = 0;
        for y in 0..map.len() {
            for x in 0..map[0].len() {
                if map[y][x] == 'O' {
                    load += height - y;
                }
            }
        }
        load
    }

    fn part1(&mut self) -> usize{
        let mut map = self.map.clone();
        self.apply_gravity(&mut map, (0, -1));
        self.calculate_load(&map)
    }

    fn map_to_string(&self, map: &Vec<Vec<char>>) -> String {
        map.iter().map(|row| row.iter().collect::<String>()).join("")
    }

    fn part2(&mut self) -> usize {
        let target_cycles = 1000000000;

        let mut map = self.map.clone();
        let mut cycles: BiMap<String, usize> = BiMap::new();

        // Detect cycle
        let mut cycle = 0;
        let (cycle_start, cycle_end) = loop {
            let map_str = self.map_to_string(&map);
            if let Some(cycle_start) = cycles.get_by_left(&map_str) {
                // We found a cycle
                break (cycle_start, cycle)
            } else {
                cycles.insert(map_str, cycle);
                self.apply_cycle(&mut map);
                cycle += 1;
            }
        };

        let end_iter = cycle_start + (target_cycles - cycle_start) % (cycle_end - cycle_start);
        let end_map = cycles.get_by_right(&end_iter).unwrap();
        let map = end_map.chars().collect_vec().chunks(self.lines[0].len()).map(|line| line.iter().map(|c| *c).collect_vec()).collect_vec();
        self.calculate_load(&map)
    }

    pub fn solve(&mut self) {
        println!("========= DAY 14 ========");
        
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