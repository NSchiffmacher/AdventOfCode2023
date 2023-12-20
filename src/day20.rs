use std::fs::read_to_string;
use std::io::{self, Write};

use std::collections::{HashMap, VecDeque};

pub struct Solution {
    lines: Vec<String>,
    modules: HashMap<String, Module>,
    broadcaster_destinations: Vec<String>,
    broadcaster_signals: VecDeque<(String, String, bool)>,
    rx_prec: String,
}

const PRECEDING_MODULES: [&str; 4] = ["jt", "sx", "kb", "ks"];

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        let mut modules = HashMap::new();
        let mut broadcaster_destinations = Vec::new();
        let mut rx_prec = String::new();

        for line in read_to_string("inputs/day20.txt").unwrap().lines() {
            let line = line.to_string();

            // Parse input
            let module = Module::from(line.as_str());
            if let Module::Broadcaster{ destinations } = module {
                broadcaster_destinations = destinations;
            } else {
                let label = match &module {
                    Module::Broadcaster{..} => panic!("Broadcaster module should not be in the map"),
                    Module::FlipFlop{ label, .. } => label,
                    Module::Conjunction{ label, .. } => label,
                };
                let destinations = match &module {
                    Module::Broadcaster{..} => panic!("Broadcaster module should not be in the map"),
                    Module::FlipFlop{ destinations, .. } => destinations,
                    Module::Conjunction{ destinations, .. } => destinations,
                };
                if destinations.contains(&"rx".to_string()) {
                    rx_prec = label.clone();
                }
                modules.insert(label.clone(), module);
            }

            
            lines.push(line);
        }
        
        // Compute button press pulses
        let mut broadcaster_signals: VecDeque<(String, String, bool)> = VecDeque::new();
        let pulse = false;
        for destination in &broadcaster_destinations {
            broadcaster_signals.push_back(("broadcaster".to_string(), destination.clone(), pulse));
        }


        // Set the last_pulses to false
        let origins = modules.keys().map(|s| s.clone()).collect::<Vec<_>>();
        for origin in origins {
            let destinations = match &modules.get(&origin).unwrap() {
                Module::Broadcaster{ destinations } => destinations,
                Module::FlipFlop{ destinations, .. } => destinations,
                Module::Conjunction{ destinations, .. } => destinations,
            }.clone();
            for destination in destinations {
                if let Some(Module::Conjunction{ last_pulses, .. }) = modules.get_mut(&destination) {
                    last_pulses.insert(origin.clone(), false);
                }
            }
        }

        Self {
            lines,
            modules,
            broadcaster_destinations,
            broadcaster_signals,
            rx_prec,
        }
    }

    fn push_button(signals: &mut VecDeque<(String, String, bool)>, modules: &mut HashMap<String, Module>, low_pulses: &mut usize, high_pulses: &mut usize, button_press: usize, cycles: &mut HashMap<String, usize>) {
        // Assume that signals contains the signals generated by the buttons
        while let Some((origin, destination, pulse)) = signals.pop_front() {
            // println!("{} -{}> {}", origin, pulse, destination);
            // if origin == "inv" && destination == "a" && !pulse {
                //     return;
                // }

            // PART 1 
            if pulse {
                *high_pulses += 1;
            } else {
                *low_pulses += 1;
            }

            // Part 2 
            if PRECEDING_MODULES.contains(&origin.as_str()) && pulse && !cycles.contains_key(&origin){
                // cycles.push((button_press, origin.clone()));
                cycles.insert(origin.clone(), button_press);
            }

            let module = modules.get_mut(&destination);
            if module.is_none() {
                continue;
            } 

            let module = module.unwrap();

            match module {
                Module::Broadcaster{ destinations: _ } => {
                    panic!("Broadcaster module should not receive a signal");
                },
                Module::FlipFlop{ label, memory, destinations } => {
                    if pulse == false {
                        *memory = !*memory;
                        for next_destination in destinations.iter() {
                            // if *memory && ["jt", "sx", "kb", "ks"].contains(&next_destination.as_str()) {
                            //     cycles.push(button_press);
                            // }
                            signals.push_back((label.clone(), next_destination.clone(), *memory));
                        }
                    }
                },
                Module::Conjunction{ label, last_pulses, destinations } => {
                    let old = last_pulses.get_mut(&origin).unwrap();
                    *old = pulse;

                    let output_pulse = !last_pulses.values().all(|&p| p == true);
                    for next_destination in destinations.iter() {
                        signals.push_back((label.clone(), next_destination.clone(), output_pulse));
                    }
                },
            
            }
        }
    }

    fn part1(&mut self) -> usize {
        let mut modules = self.modules.clone();
        let mut broadcaster_signals: VecDeque<(String, String, bool)> = VecDeque::new();
        let pulse = false;
        for destination in self.broadcaster_destinations.iter() {
            broadcaster_signals.push_back(("broadcaster".to_string(), destination.clone(), pulse));
        }

        let mut low_pulses = 0;
        let mut high_pulses = 0;

        // Press button once
        for _press in 0..1000 {
            low_pulses += 1; // button press
            Self::push_button(&mut broadcaster_signals.clone(), &mut modules, &mut low_pulses, &mut high_pulses, 0, &mut HashMap::new());
        }

        low_pulses * high_pulses
    }

    fn part2(&mut self) -> usize {
        // Button press signals
        let mut modules = self.modules.clone();
        let mut broadcaster_signals: VecDeque<(String, String, bool)> = VecDeque::new();
        let pulse = false;
        for destination in self.broadcaster_destinations.iter() {
            broadcaster_signals.push_back(("broadcaster".to_string(), destination.clone(), pulse));
        }

        let mut cycles = HashMap::new();
        let mut button_presses = 0;
        while cycles.len() != PRECEDING_MODULES.len() {
            button_presses += 1;
            Self::push_button(&mut broadcaster_signals.clone(), &mut modules, &mut 0, &mut 0, button_presses, &mut cycles);
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

#[derive(Debug, Clone)]
enum Module {
    Broadcaster{ destinations: Vec<String> },
    FlipFlop{ label: String, memory: bool, destinations: Vec<String>},
    Conjunction{ label: String, last_pulses: HashMap<String, bool>, destinations: Vec<String>},
}

impl From<&str> for Module {
    fn from(value: &str) -> Self {
        let (start, end) = value.split_once(" -> ").unwrap();
        let destinations = end.split(", ").map(|s| s.to_string()).collect::<Vec<_>>();
        if start.starts_with("%") {
            // Flip flop
            let label = start[1..].to_string();
            Self::FlipFlop{ label, memory: false, destinations }
        } else if start.starts_with("&") {
            let label = start[1..].to_string();
            Self::Conjunction { label, last_pulses: HashMap::new(), destinations }
        } else if start == "broadcaster" {
            Self::Broadcaster { destinations }
        } else {
            panic!("Unknown module type: {}", start);
        }
    }
}