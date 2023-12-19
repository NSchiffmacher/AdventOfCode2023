use std::collections::{HashMap, VecDeque};
use std::fs::read_to_string;
use std::io::{self, Write};

use itertools::Itertools;

pub struct Solution {
    workflows: HashMap<String, Workflow>,
    parts: Vec<Part>,
}

impl Solution {
    pub fn init() -> Self {
        let content = read_to_string("inputs/day19.txt").unwrap();

        let (workflows_str, parts_str) = content.split_once("\n\n").unwrap();

        // Parse the workflows
        let mut workflows = HashMap::new();
        for workflow in workflows_str.lines() {
            let (name, rules_str) = workflow.split_once("{").unwrap();
            let rules = rules_str.strip_suffix("}").unwrap().split(",").map(
                |rule_str| Rule::from(rule_str)
            ).collect_vec();
            workflows.insert(name.to_string(), Workflow::new(rules));
        }

        // Parse the parts
        let parts = parts_str.lines().map(
            |part_str| Part::from(part_str)
        ).collect_vec();

        Self {
            workflows,
            parts,
        }
    }

    fn part1(&mut self) -> usize {
        let mut res = 0;
        for part in &self.parts {
            let mut position = "in".to_string();
            while position != "R" && position != "A" {
                let workflow = &self.workflows[&position];
                position = workflow.apply(part);
            }

            if position == "A" {
                res += part.x + part.a + part.m + part.s;
            }
        }

        res
    }


    fn part2(&mut self) -> usize {
        let mut parts = VecDeque::new();
        parts.push_front(("in".to_string(), PartInterval {
            x: Interval::new(1, 4000),
            m: Interval::new(1, 4000),
            a: Interval::new(1, 4000),
            s: Interval::new(1, 4000),
        }));

        let mut accepted_parts = vec![];

        while let Some((position, part)) = parts.pop_front() {
            let workflow = &self.workflows[&position];
            
            for (new_position, new_part) in workflow.apply_interval(&part) {
                if new_position == "A" {
                    accepted_parts.push(new_part);
                } else if new_position != "R" {
                    parts.push_back((new_position, new_part));
                }
            }
        }

        // Now we need to count, hopping that the interval don't overlap otherwise I'll start crying
        let mut res = 0;
        for part in accepted_parts {
            res += (part.a.max - part.a.min + 1)
                * (part.m.max - part.m.min + 1)
                * (part.s.max - part.s.min + 1) 
                * (part.x.max - part.x.min + 1);
                
        }
        res
    }

    pub fn solve(&mut self) {
        println!("========= DAY 19 ========");
        
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

#[derive(Debug)]
struct Rule {
    condition: Condition,
    result: String,
}

impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        if let Some((start, result)) = value.split_once(":") {
            let chars = start.chars().collect_vec();

            let label = chars[0];
            let condition_char = chars[1];
            let value = start[2..].parse::<usize>().unwrap();
            let condition = match condition_char {
                '>' => Condition::GreaterThan(label, value),
                '<' => Condition::LessThan(label, value),
                _ => unreachable!("Invalid condition"),
            };


            Self {
                condition,
                result: result.to_string(),
            }
        } else {
            Self {
                condition: Condition::AlwaysTrue,
                result: value.to_string(),
            }
        }

    }
}

#[derive(Debug)]
enum Condition {
    GreaterThan(char, usize),
    LessThan(char, usize),
    AlwaysTrue,
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
}

impl Workflow {
    fn new(rules: Vec<Rule>) -> Self {
        Self {
            rules,
        }
    }
    
    fn apply(&self, part: &Part) -> String {
        for rule in &self.rules {
            // Check if the rule applyes
            match rule.condition {
                Condition::AlwaysTrue => return rule.result.clone(),
                Condition::GreaterThan(label, value) => {
                    if part.get(label) > value {
                        return rule.result.clone();
                    }
                },
                Condition::LessThan(label, value) => {
                    if part.get(label) < value {
                        return rule.result.clone();
                    }
                },
            }
        }

        unreachable!("Error when applying workflow {:?} on part {:?}", self, part);
    }

    fn apply_interval(&self, part: &PartInterval) -> Vec<(String, PartInterval)> {
        let mut res = vec![];
        let mut part = part.clone();

        for rule in &self.rules {
            // Check if the rule applyes
            match rule.condition {
                Condition::AlwaysTrue => res.push((rule.result.clone(), part.clone())),
                Condition::GreaterThan(label, value) => {
                    // Check si on est censé se faire split
                    let interval = part.get(label);
                    if value > interval.max {
                        continue;
                    }

                    if value < interval.min {
                        res.push((rule.result.clone(), part.clone()))
                    }

                    let passe = Interval::new(value+1, interval.max);
                    let mut part_split = part.clone();
                    part_split.set(label, passe);
                    res.push((rule.result.clone(), part_split));

                    part.set(label, Interval::new(interval.min, value))
                },
                Condition::LessThan(label, value) => {
                    let interval = part.get(label);

                    if value < interval.min {
                        continue
                    }

                    if value > interval.max {
                        res.push((rule.result.clone(), part.clone()))
                    }

                    let passe = Interval::new(interval.min, value-1);
                    let mut part_split = part.clone();
                    part_split.set(label, passe);
                    res.push((rule.result.clone(), part_split));

                    part.set(label, Interval::new(value, interval.max));
                }
            }
        }

        res
    }
}

#[derive(Debug, Clone)] 
struct PartInterval {
    x: Interval,
    m: Interval,
    a: Interval,
    s: Interval,
}

impl PartInterval {
    fn set(&mut self, label: char, value: Interval) {
        match label {
            'x' => self.x = value,
            'm' => self.m = value,
            'a' => self.a = value,
            's' => self.s = value,
            _ => unreachable!("Invalid label"),
        }
    }

    fn get(&self, label: char) -> Interval {
        match label {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => unreachable!("Invalid label"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Interval {
    min: usize,
    max: usize,
}

impl Interval {
    fn new(min: usize, max: usize) -> Self {
        Self {
            min,
            max,
        }
    }

    fn contains(&self, value: usize) -> bool {
        self.min <= value && value <= self.max
    }

    fn split_on(&self, value: usize) -> (Interval, Interval) {
        assert!(self.contains(value));

        (Self::new(self.min, value-1), Self::new(value + 1, self.max))
    }

}

#[derive(Debug)]
struct Part {
    x: usize,
    m: usize,
    a: usize,
    s: usize,
}

impl From<&str> for Part {
    fn from(value: &str) -> Self {
        let mut res = Self {
            x: 0,
            m: 0,
            a: 0,
            s: 0,
        };

        for part in value.strip_prefix("{").unwrap().strip_suffix("}").unwrap().split(",") {
            let (label, value) = part.split_once("=").unwrap();
            res.set(label.chars().next().unwrap(), value.parse::<usize>().unwrap());
        }

        res
    }
}

impl Part {
    fn set(&mut self, label: char, value: usize) {
        match label {
            'x' => self.x = value,
            'm' => self.m = value,
            'a' => self.a = value,
            's' => self.s = value,
            _ => unreachable!("Invalid label"),
        }
    }

    fn get(&self, label: char) -> usize {
        match label {
            'x' => self.x,
            'm' => self.m,
            'a' => self.a,
            's' => self.s,
            _ => unreachable!("Invalid label"),
        }
    }
}