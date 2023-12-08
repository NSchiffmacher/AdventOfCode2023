use std::fs::read_to_string;
use std::io::{self, Write};

use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;

pub struct Solution {
    lines: Vec<String>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day8.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        Self {
            lines,
        }
    }

    fn part1(&mut self) -> usize {
        let instructions = self.lines[0].chars().map(|c| Instruction::from(c)).collect_vec();
        let tree = Tree::from(&self.lines);

        let num_instructions = instructions.len();
        let mut instructions_counter = 0;
        let mut current_node = "AAA";
        let goal = "ZZZ";

        while current_node != goal {
            let node = &tree.nodes[current_node];
            current_node = match instructions[instructions_counter % num_instructions] {
                Instruction::Left => &node.left,
                Instruction::Right => &node.right,
            };
            instructions_counter += 1;
        }

        instructions_counter
    }

    fn part2(&mut self) -> usize {
        let instructions = self.lines[0].chars().map(|c| Instruction::from(c)).collect_vec();
        let tree = Tree::from(&self.lines);

        tree
            .nodes
            .keys()
            .filter(|name| name.ends_with('A'))
            .map(|start_node| {
                let mut current_node = start_node;
                let mut instructions_counter = 0;
                while !current_node.ends_with('Z') {
                    let node = &tree.nodes[current_node];
                    current_node = match instructions[instructions_counter % instructions.len()] {
                        Instruction::Left => &node.left,
                        Instruction::Right => &node.right,
                    };
                    instructions_counter += 1;
                }
                instructions_counter
            })
            .fold(1, |acc, n| num::integer::lcm(acc, n))
    }

    pub fn solve(&mut self) {
        println!("========= DAY 8 ========");
        print!("Solving part 1: ");
        io::stdout().flush().unwrap();
        println!("{:?}", self.part1());
        
        print!("Solving part 2: ");
        io::stdout().flush().unwrap();
        println!("{:?}\n", self.part2());
    }
}


#[derive(Debug)]
pub struct Node {
    value: String,
    left: String,
    right: String,
}

impl Node {
    fn new(value: String, left: String, right: String) -> Self {
        Self { value, left, right }
    }
}

#[derive(Debug)]
pub struct Tree {
    nodes: HashMap<String, Node>,
}

impl From<&Vec<String>> for Tree {
    fn from(lines: &Vec<String>) -> Self {
        let regex = Regex::new(r"(\w+) = \((\w+), (\w+)\)").unwrap();
        let nodes = lines[2..].iter().fold(HashMap::new(), |mut map, line| {
            let captures = regex.captures(line).unwrap();
            let node = Node::new(captures[1].to_string(), captures[2].to_string(), captures[3].to_string());
            map.insert(captures[1].to_string(), node);
            map
        });
        Self { nodes }
    }
}

enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Instruction::Left,
            'R' => Instruction::Right,
            _ => panic!("Unknown instruction {}", value),
        }
    }
}