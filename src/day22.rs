use std::collections::HashSet;
use std::fs::read_to_string;
use std::io::{self, Write};

use itertools::Itertools;

pub struct Solution {
    lines: Vec<String>,
    bricks: Vec<Brick>,
    cannot_be_deintegrated: Vec<usize>,
    supports: Vec<HashSet<usize>>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        let mut bricks = Vec::new();
        for (i, line) in read_to_string("inputs/day22.txt").unwrap().lines().enumerate() {
            bricks.push(Brick::parse(line, i));
            lines.push(line.to_string());
        }

        Self {
            lines,
            bricks,
            cannot_be_deintegrated: vec![],
            supports: vec![],
        }
    }

    fn part1(&mut self) -> usize {
        // First step is to make them fall
        // 1) Let's compute, for each piece, the piece with the lowest z value, it will be the ordering to make the pieces fall
        let idx = self.bricks.iter().sorted_unstable_by_key(|brick| brick.lowest_z()).map(|brick| brick.id).collect::<Vec<_>>();
        
        // Let's have a hashmap that maps, for every (x, y) coordinates, the current max z and the brick id
        let mut height_map: std::collections::HashMap<(usize, usize), (usize, usize)> = std::collections::HashMap::new();
        
        // array that maps every id to all the different pieces that are on top of it
        let mut supports: Vec<HashSet<usize>> = vec![HashSet::new(); self.bricks.len() + 1]; // + 1 for ground

        for id in &idx {
            // For every brick, we need to check if it's resting on something
            let mut resting_on = HashSet::new();
            loop{ // Piece must keep falling until it lands on something
                for (x, y, z) in self.bricks[*id].blocks() {
                    // Check if at z = z - 1 there are some pieces we are resting on
                    let bellow = *height_map.get(&(x, y)).unwrap_or(&(0usize, self.bricks.len())); // Ground is an unused id: the len of the array
                    if z - 1 == bellow.0 { // Resting on that piece
                        resting_on.insert(bellow.1);
                    } 
                }

                if resting_on.len() == 0 { 
                    self.bricks[*id].fall();
                } else {
                    // We are resting on some stuff, we can stop there.
                    // The current piece needs to update the max z value of all it's (x, y, _) cubes, and needs to say on what it's resting
                    for (x, y, z) in self.bricks[*id].blocks() {
                        if let Some((curr_z, curr_id)) = height_map.get_mut(&(x, y)) {
                            // A value already exists, we need to update z if our z is higher
                            if z > *curr_z {
                                *curr_z = z;
                                *curr_id = *id;
                            }
                        } else {
                            // We were actually resting on the ground
                            height_map.insert((x, y), (z, *id));
                        }
                    }

                    // And we can also set it backward (read next comment first)
                    for id2 in &resting_on {
                        supports[*id2].insert(*id);
                    }

                    // Now we can set on what we are resting
                    self.bricks[*id].resting_on = resting_on.iter().cloned().collect_vec();

                    break;
                }
            }
        }

        let mut res = 0;
        for id in 0..self.bricks.len() {
            // We can remove a brick if, for all pieces standing on it there is at least one other piece carrying it
            // aka if for all brick in supports[id], len(resting_on) > 1
            // or if it doesn't support any brick

            if supports[id].iter().all(|brick_id| self.bricks[*brick_id].resting_on.len() > 1) {
                res += 1;
            } else {
                // it's a part that "cannot" be deintegrated, let's store it for part 2 
                self.cannot_be_deintegrated.push(id);
            }
        }
        
        self.supports = supports;

        res
    }

    fn part2(&mut self) -> usize {
        // cache[id] = # of bricks that can be deintegrated if we remove brick id
        let supported_by = self.bricks.iter().map(|brick| brick.resting_on.clone()).collect_vec();

        let mut res = 0;
        for id in &self.cannot_be_deintegrated {
            // Could store results for subproblems, but it's fast enough
            let mut tmp_res = 0;
            let mut supported_by = supported_by.clone();
            let mut queue = vec![*id];
            while let Some(id) = queue.pop() {
                tmp_res += 1;
                for brick_id in &self.supports[id] {
                    supported_by[*brick_id].retain(|x| *x != id);
                    if supported_by[*brick_id].len() == 0 {
                        queue.push(*brick_id);
                    }
                }
            }

            res += tmp_res - 1;
        }

        res
    }

    pub fn solve(&mut self) {
        println!("========= DAY 22 ========");
        
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

#[derive(Debug, Clone)]
struct Brick {
    id: usize,
    blocks: Vec<(usize, usize, usize)>,
    resting_on: Vec<usize>, // Vec of ids
    fall_distance: usize,
}

impl Brick {
    fn lowest_z(&self) -> usize {
        *self.blocks.iter().map(|(_, _, z)| z).min().unwrap() - self.fall_distance
    }

    fn highest_z(&self) -> usize {
        *self.blocks.iter().map(|(_, _, z)| z).max().unwrap() - self.fall_distance
    }

    fn fall(&mut self) {
        self.fall_distance += 1;
    }

    fn blocks<'a>(&'a self) -> impl std::iter::Iterator<Item = (usize, usize, usize)> + 'a {
        self.blocks.iter().map(|(x, y, z)| (*x, *y, *z - self.fall_distance))
    }

    fn parse(value: &str, id: usize) -> Self {
        let (start, end) = value.split_once("~").unwrap();
        let start = start.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();
        let end = end.split(",").map(|x| x.parse::<usize>().unwrap()).collect::<Vec<_>>();

        let x1 = start[0].min(end[0]);
        let x2 = start[0].max(end[0]);

        let y1 = start[1].min(end[1]);
        let y2 = start[1].max(end[1]);

        let z1 = start[2].min(end[2]);
        let z2 = start[2].max(end[2]);

        let mut blocks = vec![];
        for x in x1..=x2 {
            for y in y1..=y2 {
                for z in z1..=z2 {
                    blocks.push((x, y, z));
                }
            }
        }

        Self {
            id,
            blocks,
            resting_on: vec![],
            fall_distance: 0,
        }
    }
}