use std::fs::read_to_string;
use std::io::{self, Write};

use std::collections::HashSet;

pub struct Solution {
    lines: Vec<String>,
    cards: Vec<Card>,
}

struct Card {
    winning_numbers: HashSet<u32>,
    numbers: HashSet<u32>,
    in_common: usize,
}

impl From<&str> for Card {
    fn from(line: &str) -> Self {
        let (winning_numbers, numbers) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let winning_numbers: HashSet<_> = winning_numbers.trim().split(" ").filter_map(|s| s.trim().parse::<u32>().ok()).collect();
        let numbers: HashSet<_> = numbers.trim().split(" ").filter_map(|s| s.trim().parse::<u32>().ok()).collect();

        Self {
            in_common: winning_numbers.intersection(&numbers).count(),
            winning_numbers, 
            numbers,
        }
    }
}

impl Card {
    fn score(&self) -> u32 {
        if self.in_common > 0 {
            2u32.pow(self.in_common as u32 - 1)
        } else {
            0
        }
    }

    fn num_matching(&self) -> usize {
        self.in_common
    }
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day4.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        Self {
            cards: lines.iter().map(|card_str| Card::from(card_str.as_str())).collect(),
            lines,
        }
    }

    fn part1(&mut self) -> u32 {
        self.cards.iter().map(|card| card.score()).sum()
    }

    fn part2(&mut self) -> i32 {
        let mut num_cards = vec![1; self.cards.len()];

        for card_number in 0..self.cards.len() {
            let num_matching = &self.cards[card_number].num_matching();
            let amount = num_cards[card_number];

            for next_card in card_number+1..card_number+1+num_matching {
                num_cards[next_card] += amount;
            }
        }

        num_cards.iter().sum::<i32>()
    }

    pub fn solve(&mut self) {
        println!("========= DAY 4 ========");
        print!("Solving part 1: ");
        io::stdout().flush().unwrap();
        println!("{:?}", self.part1());
        
        print!("Solving part 2: ");
        io::stdout().flush().unwrap();
        println!("{:?}\n", self.part2());
    }
}