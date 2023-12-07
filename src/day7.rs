use std::fs::read_to_string;
use std::io::{self, Write};

use std::collections::HashMap;
use itertools::Itertools;

pub struct Solution {
    lines: Vec<String>,
}

impl Solution {
    pub fn init() -> Self {
        let mut lines = Vec::new();
        for line in read_to_string("inputs/day7.txt").unwrap().lines() {
            lines.push(line.to_string());
        }

        Self {
            lines,
        }
    }

    fn part1(&mut self) -> usize {
        self.lines
            .iter()
            .map(|hand_str| Hand::from(hand_str.as_str()))
            .sorted()
            .enumerate()
            .map(|(rank, hand)| (rank+1) * hand.bid)
            .sum()
        
    }

    fn part2(&mut self) {

    }

    pub fn solve(&mut self) {
        println!("========= DAY 7 ========");
        print!("Solving part 1: ");
        io::stdout().flush().unwrap();
        println!("{:?}", self.part1());
        
        print!("Solving part 2: ");
        io::stdout().flush().unwrap();
        println!("{:?}\n", self.part2());
    }
}

#[derive(Debug, PartialEq, Ord, Eq)]
struct Hand {
    cards: [Card; 5],
    bid: usize,
    hand_type: HandType,
}

impl From<&str> for Hand {
    fn from(line: &str) -> Self {
        let (cards, bid) = line.split_once(" ").unwrap();
        let cards = cards.chars().map(|card| Card::from(card)).collect_vec();
        let bid = bid.parse().unwrap();

        // Determine card type
        let frequencies_map = cards.iter().fold(HashMap::new(), |mut map, val|{
            map.entry(val)
               .and_modify(|frq|*frq+=1)
               .or_insert(1);
            map
        });
        let frequencies = &frequencies_map.values().sorted().rev().map(|x| *x).collect_vec()[..];
        let hand_type = match frequencies {
            [5] => HandType::FiveOfAKind,
            [4, 1] => HandType::FourOfAKind,
            [3, 2] => HandType::FullHouse,
            [3, 1, 1] => HandType::ThreeOfAKind,
            [2, 2, 1] => HandType::TwoPair,
            [2, 1, 1, 1] => HandType::OnePair,
            [1, 1, 1, 1, 1] => HandType::HighCard,
            _ => panic!("Impossible frequencies: {:?} for frequencies map {:?}", frequencies, frequencies_map),
        };

        Self {
            cards: cards.try_into().unwrap(),
            bid,
            hand_type,
        }

    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.cmp(&other.hand_type) {
            std::cmp::Ordering::Equal => {
                // If equal, compare the cards themselves
                for i in 0..5 {
                    let a = &self.cards[i];
                    let b = &other.cards[i];

                    match a.cmp(&b) {
                        std::cmp::Ordering::Equal => (),
                        ordering => return Some(ordering),
                    };
                }
                Some(std::cmp::Ordering::Equal)
            }
            ordering => Some(ordering),
        }
    }
}


#[derive(Debug, PartialEq, PartialOrd, Ord, Eq)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
enum Card {
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Height,
    Nine,
    T,
    J,
    Q,
    K,
    A,
}

impl From<char> for Card {
    fn from(value: char) -> Self {
        match value {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Height,
            '9' => Self::Nine,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'A' => Self::A,
            'K' => Self::K,
            _ => panic!("Unknown card {}", value),
        }
    }
}