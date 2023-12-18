use std::fs::read_to_string;
use std::io::{self, Write};
use itertools::Itertools;

use std::collections::{HashMap, HashSet, VecDeque};

pub struct Solution {
    lines: Vec<String>,
    map: Vec<Vec<char>>,
    char_to_direction: HashMap<char, Vec<(i32, i32)>>,
    direction_to_pipes: HashMap<(i32, i32), HashSet<char>>,
    start: (i32, i32),
    width: i32,
    height: i32,
    path: HashSet<(i32, i32)>,
}



impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        // INPUT NEEDS TO BE CHANGED: CHANGE START BY ACTUAL PIPE FORMAT, ADD START POSITION AT THE TOP OF THE FILE
        // SOMETIMES ADD PADDING TOP/BOTTOM + UPDATE STARTY ACCORDINGLY...
        for line in read_to_string("inputs/day10.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        let start: (i32, i32) = lines[0].split(",").map(|x| x.parse::<i32>().unwrap()).collect_tuple().unwrap();
        let map = lines.iter().skip(1).map(|line| line.chars().collect_vec()).collect_vec();

        let mut char_to_direction: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
        char_to_direction.insert('S', vec![(1, 0), (-1, 0), (0, 1), (0, -1)]);
        char_to_direction.insert('|', vec![(0, 1), (0, -1)]);
        char_to_direction.insert('-', vec![(1, 0), (-1, 0)]);
        char_to_direction.insert('L', vec![(0, -1), (1, 0)]);
        char_to_direction.insert('J', vec![(0, -1), (-1, 0)]);
        char_to_direction.insert('7', vec![(0, 1), (-1, 0)]);
        char_to_direction.insert('F', vec![(0, 1), (1, 0)]);

        let mut direction_to_pipes: HashMap<(i32, i32), HashSet<char>> = HashMap::new();
        direction_to_pipes.insert((0, -1), HashSet::from(['|', '7', 'F', 'S']));
        direction_to_pipes.insert((0, 1),  HashSet::from(['|', 'J', 'L', 'S']));
        direction_to_pipes.insert((1, 0),  HashSet::from(['-', '7', 'J', 'S']));
        direction_to_pipes.insert((-1, 0), HashSet::from(['-', 'F', 'L', 'S']));

        Self {
            width: map[0].len() as i32,
            height: map.len() as i32,
            lines,
            map,
            char_to_direction,
            direction_to_pipes,
            start,
            path: HashSet::new(),
        }
    }

    fn get_char(&self, pos: (i32, i32)) -> char {
        self.map[pos.1 as usize][pos.0 as usize]
    }

    fn find_neighboors(&self, from: (i32, i32)) -> HashSet<(i32, i32)> {
        let mut res = HashSet::new();

        let from_char = self.get_char(from);

        for direction in &self.char_to_direction[&from_char]{
            let possible_pipes = &self.direction_to_pipes[direction];
            let new_position = (from.0 + direction.0, from.1 + direction.1);
            if new_position.0 >= 0 && new_position.0 < self.width && new_position.1 >= 0 && new_position.1 < self.height && possible_pipes.contains(&self.get_char(new_position)) {
                res.insert(new_position);
            } 
        }

        res
    }

    fn part1(&mut self) -> usize{
        let mut costs: HashMap<(i32, i32), usize> = HashMap::new();
        costs.insert(self.start, 0);

        // Run BFS
        let mut queue = VecDeque::new();
        queue.push_front(self.start);
        self.path.insert(self.start);

        while let Some(position) = queue.pop_front() {
            let cost = costs[&position];
            for neighboor in self.find_neighboors(position) {
                if !self.path.contains(&neighboor) {
                    costs.insert(neighboor, cost + 1);
                    queue.push_back(neighboor);
                    self.path.insert(neighboor);
                }
            }
        }

        let max_entry = costs.iter().max_by_key(|(_, cost)| *cost).unwrap();
        *max_entry.1

    }

    fn part2(&mut self) -> usize {
        let dir_change_chars = vec!['F', 'J', '7', 'L'];
        let mut direction_changers: HashMap<(char, (i32, i32)), (i32, i32)> = HashMap::new();
        direction_changers.insert(('F', (0, -1)), (1, 0));
        direction_changers.insert(('F', (-1, 0)), (0, 1));
        direction_changers.insert(('7', (0, -1)), (-1, 0));
        direction_changers.insert(('7', (1, 0)), (0, 1));
        direction_changers.insert(('J', (0, 1)), (-1, 0));
        direction_changers.insert(('J', (1, 0)), (0, -1));
        direction_changers.insert(('L', (0, 1)), (1, 0));
        direction_changers.insert(('L', (-1, 0)), (0, -1));

        // Go through the path and compute two different sides
        let mut direction = (0, -1);
        let first = self.path.iter().find(|pos| self.get_char(**pos) == '|').unwrap().clone();
        let mut current = (first.0 + direction.0, first.1 + direction.1);

        let mut side1 = HashSet::new();
        let mut side2 = HashSet::new();

        while current != first {
            let c = self.get_char(current);

            if direction.1 == 0 {
                // Do like a '-'
                if !self.path.contains(&(current.0, current.1 - 1 * direction.0)) {
                    side1.insert((current.0, current.1 - 1 * direction.0));
                }
                if !self.path.contains(&(current.0, current.1 + 1 * direction.0)) {
                    side2.insert((current.0, current.1 + 1 * direction.0));
                }
            } else {
                // Do like a '|'
                if !self.path.contains(&(current.0 + 1 * direction.1, current.1)) {
                    side1.insert((current.0 + 1 * direction.1, current.1)); // signs swap bc -1 is up
                }
                if !self.path.contains(&(current.0 - 1 * direction.1, current.1)) {
                    side2.insert((current.0 - 1 * direction.1, current.1));
                } 
            }

            // Change the direction
            if dir_change_chars.contains(&c) {
                direction = direction_changers[&(c, direction)];

                // Do it again
                if direction.1 == 0 {
                    // Do like a '-'
                    if !self.path.contains(&(current.0, current.1 - 1 * direction.0)) {
                        side1.insert((current.0, current.1 - 1 * direction.0));
                    }
                    if !self.path.contains(&(current.0, current.1 + 1 * direction.0)) {
                        side2.insert((current.0, current.1 + 1 * direction.0));
                    }
                } else {
                    // Do like a '|'
                    if !self.path.contains(&(current.0 + 1 * direction.1, current.1)) {
                        side1.insert((current.0 + 1 * direction.1, current.1)); // signs swap bc -1 is up
                    }
                    if !self.path.contains(&(current.0 - 1 * direction.1, current.1)) {
                        side2.insert((current.0 - 1 * direction.1, current.1));
                    } 
                }
            }
            current = (current.0 + direction.0, current.1 + direction.1);
        }

        // Now we must figure out which one is outside, and which one is outside
        // Let's find the one the most above, must be outside
        let side1_min = side1.iter().min_by_key(|(_, y)| y).unwrap_or(&(0, 9999)).1;
        let side2_min = side2.iter().min_by_key(|(_, y)| y).unwrap_or(&(0, 9999)).1;

        let (mut inside, _outside) = if side1_min < side2_min {
            (side2, side1)
        } else {
            (side1, side2)
        };

        // From all the points "inside", run a kind of percolation/DFS to expand the inside
        let mut queue = VecDeque::from_iter(inside.iter().cloned());
        while let Some(position) = queue.pop_front() {
            for (dx, dy) in [(0, -1), (0, 1), (-1, 0), (1, 0)] {
                let new_position = (position.0 + dx, position.1 + dy);
                if !inside.contains(&new_position) && new_position.0 >= 0 && new_position.0 < self.width && new_position.1 >= 0 && new_position.1 < self.height {
                    if !self.path.contains(&new_position) {
                        inside.insert(new_position);
                        queue.push_back(new_position);
                    }
                }

            }
        }
        inside.len()
    }

    pub fn solve(&mut self) {
        println!("========= DAY 10 ========");
        
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