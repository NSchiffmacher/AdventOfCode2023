use std::fs::read_to_string;
use std::io::{self, Write};

pub struct Solution {
    lines: Vec<String>,
    games: Vec<Game>,
}

#[derive(Debug)]
pub struct Draw {
    pub r: u32,
    pub g: u32,
    pub b: u32,
}

impl Draw {
    pub fn new(r: u32, g: u32, b: u32) -> Self {
        Self {
            r, g, b
        }
    }
}

impl From<&str> for Draw {
    fn from(value: &str) -> Self {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for s in value.split(", ") {
            let (digit, str_identifier) = s.split_once(" ").unwrap();
            let digit = digit.parse::<u32>().unwrap();
            match str_identifier {
                "blue" => b = digit,
                "red" => r = digit,
                "green" => g = digit,
                _ => panic!("Color {} not found", str_identifier)
            };
        }

        Self::new(r, g, b)
    }
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub draws: Vec<Draw>,
}

impl Game {
    pub fn new(id: u32, draws: Vec<Draw>) -> Self {
        Self {
            id,
            draws
        }
    }

    pub fn can_be_played(&self, available_r: u32, available_g: u32, available_b: u32) -> bool {
        self.draws.iter().all(|draw| draw.r <= available_r && draw.g <= available_g && draw.b <= available_b)
    }

    pub fn power(&self) -> u32 {
        let required_r = self.draws.iter().max_by_key(|draw| draw.r).unwrap().r;
        let required_g = self.draws.iter().max_by_key(|draw| draw.g).unwrap().g;
        let required_b = self.draws.iter().max_by_key(|draw| draw.b).unwrap().b;

        required_r * required_b * required_g
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (identifier, draws) = value.strip_prefix("Game ").unwrap().split_once(": ").unwrap();
        let identifier: u32 = identifier.parse().unwrap();
        let draws = draws.split("; ").map(|draw_str| Draw::from(draw_str)).collect();

        Self::new(identifier, draws)
    }
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day2.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        Self {
            games: lines.iter().map(|line| Game::from(line.as_str())).collect(),
            lines,
        }
    }

    fn part1(&mut self) -> u32 {
        self
            .games
            .iter()
            .filter(|game| game.can_be_played(12, 13, 14))
            .map(|game| game.id)
            .sum()
    }

    fn part2(&mut self) -> u32 {
        self
            .games
            .iter()
            .map(|game| game.power())
            .sum()
    }

    pub fn solve(&mut self) {
        println!("========= DAY 2 ========");
        print!("Solving part 1: ");
        io::stdout().flush().unwrap();
        println!("{:?}", self.part1());
        
        print!("Solving part 2: ");
        io::stdout().flush().unwrap();
        println!("{:?}\n", self.part2());
    }
}