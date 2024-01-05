use std::{
    cmp::Ordering,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let mut hands = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let (hand, bid) = line.split_once(" ").unwrap();
        let hand = parse_hand(hand);
        let bid = bid.parse::<usize>().unwrap();
        hands.push(HandWithBid { hand, bid })
    }
    hands.sort();

    let mut total = 0;

    for (rm1, hand) in hands.iter().enumerate() {
        let rank = rm1 + 1;
        total += rank * hand.bid;
    }

    println!("{total}");
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct HandWithBid {
    hand: Hand,
    bid: usize,
}

#[derive(Debug, PartialEq, Eq, Ord)]
struct Hand(Vec<Card>);

impl Hand {
    fn kind(&self) -> HandKind {
        let mut amounts = [0; 13];
        for card in &self.0 {
            amounts[card.index()] += 1;
        }
        match amounts.iter().max().unwrap() {
            5 => HandKind::Five,
            4 => HandKind::Four,
            3 => {
                if amounts.iter().find(|x| **x == 2).is_some() {
                    HandKind::FullHouse
                } else {
                    HandKind::Three
                }
            }
            2 => {
                if amounts.iter().filter(|x| **x == 2).count() == 2 {
                    HandKind::TwoPair
                } else {
                    HandKind::Pair
                }
            }
            1 => HandKind::None,
            _ => unreachable!(),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let o = self.kind().cmp(&other.kind());
        if o.is_ne() {
            return Some(o);
        }

        for (a, b) in self.0.iter().zip(other.0.iter()) {
            let o = a.cmp(&b);
            if o.is_ne() {
                return Some(o);
            }
        }

        Some(Ordering::Equal)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandKind {
    None,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

fn parse_hand(s: &str) -> Hand {
    Hand(
        s.chars()
            .map(|c| match c {
                'A' => Card::A,
                'K' => Card::K,
                'Q' => Card::Q,
                'J' => Card::J,
                'T' => Card::T,
                '9' => Card::_9,
                '8' => Card::_8,
                '7' => Card::_7,
                '6' => Card::_6,
                '5' => Card::_5,
                '4' => Card::_4,
                '3' => Card::_3,
                '2' => Card::_2,
                _ => unreachable!(),
            })
            .collect(),
    )
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
    J,
    Q,
    K,
    A,
}

impl Card {
    fn index(&self) -> usize {
        match self {
            Card::A => 0,
            Card::K => 1,
            Card::Q => 2,
            Card::J => 3,
            Card::T => 4,
            Card::_9 => 5,
            Card::_8 => 6,
            Card::_7 => 7,
            Card::_6 => 8,
            Card::_5 => 9,
            Card::_4 => 10,
            Card::_3 => 11,
            Card::_2 => 12,
        }
    }
}
