use std::fs::read_to_string;
use std::io::{self, Write};

use std::collections::{HashMap, VecDeque};

pub struct Solution {
    lines: Vec<String>,
    modules: HashMap<String, Box<dyn Module>>,
    preceding_modules: Vec<String>, // Part 2
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        let mut modules = HashMap::new();

        for line in read_to_string("inputs/day20.txt").unwrap().lines() {
            let line = line.to_string();

            // Parse module
            let module = parse_module(line.as_str());
            modules.insert(module.label().to_string(), module);
            
            lines.push(line);
        }

        // Set the last_pulses to false
        let origins = modules.keys().map(|s| s.clone()).collect::<Vec<_>>();
        for origin in origins {
            let destinations = modules.get(&origin).unwrap().destinations().clone();
            for destination in destinations {
                if let Some(module) = modules.get_mut(&destination) {
                    module.attach_input(origin.clone());
                }
            }
        }

        // Find THE conjunction leading to rx
        let mut rx = String::new();
        for (label, module) in modules.iter() {
            if module.destinations().contains(&"rx".to_string()) {
                rx = label.clone();
                break;
            }
        }

        // Then, find its inputs
        let mut preceding_modules = Vec::new();
        for (label, module) in modules.iter() {
            if module.destinations().contains(&rx) {
                preceding_modules.push(label.clone());
            }
        }

        Self {
            lines,
            modules,
            preceding_modules
        }
    }

    fn push_button(&self, modules: &mut HashMap<String, Box<dyn Module>>, low_pulses: &mut usize, high_pulses: &mut usize, button_press: usize, cycles: &mut HashMap<String, usize>) {
        let mut signals: VecDeque<(String, String, bool)> = VecDeque::new();
        signals.push_back(("button".to_string(), "broadcaster".to_string(), false));

        while let Some((origin, destination, pulse)) = signals.pop_front() {
            // println!("{} -{}> {}", origin, pulse, destination);

            // Part 1 
            if pulse {
                *high_pulses += 1;
            } else {
                *low_pulses += 1;
            }

            // Part 2 
            if self.preceding_modules.contains(&origin) && pulse && !cycles.contains_key(&origin){
                cycles.insert(origin.clone(), button_press);
            }

            if let Some(module) = modules.get_mut(&destination) {
                let next_signals = module.process_signal(origin.clone(), pulse);
                signals.extend(next_signals)
            }
        }
    }

    fn part1(&mut self) -> usize {
        let mut modules = self.modules.clone();

        let mut low_pulses = 0;
        let mut high_pulses = 0;

        // Press button once
        for _press in 0..1000 {
            self.push_button(&mut modules, &mut low_pulses, &mut high_pulses, 0, &mut HashMap::new());
        }

        low_pulses * high_pulses
    }

    fn part2(&mut self) -> usize {
    //     // Button press signals
        let mut modules = self.modules.clone();

        let mut cycles = HashMap::new();
        let mut button_presses = 0;
        while cycles.len() != self.preceding_modules.len() {
            button_presses += 1;
            self.push_button(&mut modules, &mut 0, &mut 0, button_presses, &mut cycles);
        }
        
        cycles.values().fold(1, |acc, x| num::integer::lcm(acc, *x))
    }

    pub fn solve(&mut self) {
        println!("========= DAY 20 ========");
        
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

trait Module: ModuleClone {
    fn label(&self) -> &str;
    fn destinations(&self) -> &Vec<String>;

    fn process_signal(&mut self, origin: String, pulse: bool) -> Vec<(String, String, bool)>;

    fn attach_input(&mut self, _origin: String) {}
}

trait ModuleClone {
    fn clone_box(&self) -> Box<dyn Module>;
}

impl<T> ModuleClone for T
where
    T: 'static + Module + Clone,
{
    fn clone_box(&self) -> Box<dyn Module> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn Module> {
    fn clone(&self) -> Box<dyn Module> {
        self.clone_box()
    }
}

fn parse_module(line: &str) -> Box<dyn Module> {
    let (start, end) = line.split_once(" -> ").unwrap();
    let destinations = end.split(", ").map(|s| s.to_string()).collect::<Vec<_>>();

    if start.starts_with("%") { // Flip flop
        let label = start[1..].to_string();
        let destinations = destinations;
        let memory = false;
        Box::new(FlipFlop{ label, destinations, memory })
    } else if start.starts_with("&") { // Conjunction
        let label = start[1..].to_string();
        let destinations = destinations;
        let received_pulses = HashMap::new();
        Box::new(Conjunction{ label, destinations, received_pulses })
    } else { // Broadcaster
        let destinations = destinations;
        Box::new(Broadcaster{ destinations })
    }
}

#[derive(Debug, Clone)]
struct Broadcaster {
    destinations: Vec<String>,
}

impl Module for Broadcaster {
    fn label(&self) -> &str {
        "broadcaster"
    }

    fn destinations(&self) -> &Vec<String> {
        &self.destinations
    }

    fn process_signal(&mut self, _origin: String, _pulse: bool) -> Vec<(String, String, bool)> {
        self.destinations.iter().map(|destination| ("broadcaster".to_string(), destination.clone(), false)).collect()
    }
}

#[derive(Debug, Clone)]
struct Conjunction {
    label: String,
    destinations: Vec<String>,
    received_pulses: HashMap<String, bool>,
}

impl Module for Conjunction {
    fn label(&self) -> &str {
        &self.label
    }

    fn destinations(&self) -> &Vec<String> {
        &self.destinations
    }

    fn process_signal(&mut self, origin: String, pulse: bool) -> Vec<(String, String, bool)> {
        if let Some(received_pulse) = self.received_pulses.get_mut(&origin) {
            *received_pulse = pulse;
            let output_pulse = !self.received_pulses.values().all(|&x| x);
            self.destinations.iter().map(|destination| (self.label.clone(), destination.clone(), output_pulse)).collect()

        } else {
            panic!("Normal ça ? {} -{}> {}", origin, pulse, self.label);
        }
    }

    fn attach_input(&mut self, origin: String) {
        self.received_pulses.insert(origin, false);
    }
}

#[derive(Debug, Clone)]
struct FlipFlop {
    label: String,
    destinations: Vec<String>,
    memory: bool,
}

impl Module for FlipFlop {
    fn label(&self) -> &str {
        &self.label
    }

    fn destinations(&self) -> &Vec<String> {
        &self.destinations
    }

    fn process_signal(&mut self, _origin: String, pulse: bool) -> Vec<(String, String, bool)> {
        if !pulse {
            self.memory = !self.memory;
            self.destinations.iter().map(|destination| (self.label.clone(), destination.clone(), self.memory)).collect()
        } else {
            vec![]
        }
    }
}