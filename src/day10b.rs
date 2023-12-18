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
        // Scale the map up by 3 times
        let mut map = vec![vec!['.'; (self.width * 3) as usize]; (self.height*3) as usize];

        let mut char_scaled = HashMap::new();
        char_scaled.insert('.', vec![vec!['.','.','.'], vec!['.', '.', '.'], vec!['.', '.', '.']]);
        char_scaled.insert('|', vec![vec!['.','|','.'], vec!['.', '|', '.'], vec!['.', '|', '.']]);
        char_scaled.insert('-', vec![vec!['.','.','.'], vec!['-', '-', '-'], vec!['.', '.', '.']]);
        char_scaled.insert('L', vec![vec!['.','|','.'], vec!['.', 'L', '-'], vec!['.', '.', '.']]);
        char_scaled.insert('J', vec![vec!['.','|','.'], vec!['-', 'J', '.'], vec!['.', '.', '.']]);
        char_scaled.insert('7', vec![vec!['.','.','.'], vec!['-', '7', '.'], vec!['.', '|', '.']]);
        char_scaled.insert('F', vec![vec!['.','.','.'], vec!['.', 'F', '-'], vec!['.', '|', '.']]);
        char_scaled.insert('.', vec![vec!['.','.','.'], vec!['.', '.', '.'], vec!['.', '.', '.']]);

        for y in 0..self.height {
            for x in 0..self.width {
                if self.path.contains(&(x, y)) {
                    let c = self.get_char((x, y));
                    let scaled_c = &char_scaled[&c];
                    for dx in 0..3 {
                        for dy in 0..3 {
                            map[(3*y + dy) as usize][(3*x + dx) as usize] = scaled_c[dy as usize][dx as usize];
                        }
                    }
                }
            }
        }

        // Run percolation outside
        let mut queue = VecDeque::new();
        queue.push_front((0, 0));
        map[0][0] = 'O';

        while let Some(position) = queue.pop_front() {
            for neighboor in [(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let new_position = (position.0 + neighboor.0, position.1 + neighboor.1);
                if new_position.0 >= 0 && new_position.0 < self.width*3 && new_position.1 >= 0 && new_position.1 < self.height*3 && map[new_position.1 as usize][new_position.0 as usize] == '.' {
                    map[new_position.1 as usize][new_position.0 as usize] = 'O';
                    queue.push_back(new_position);
                }
            }
        }


        // Count the cells with a dot, but counting only the middle ones for every 3x3 square
        let mut res = 0;
        for y in 0..self.height {
            for x in 0..self.width {
                if map[(3*y + 1) as usize][(3*x + 1) as usize] == '.' {
                    res += 1;
                }
            }
        }

        res
    }

    pub fn solve(&mut self) {
        println!("========= DAY 10 BIS ========");
        
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