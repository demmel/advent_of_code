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
        let mut kind = match amounts.iter().take(12).max().unwrap() {
            5 => HandKind::Five,
            4 => HandKind::Four,
            3 => {
                if amounts.iter().take(12).find(|x| **x == 2).is_some() {
                    HandKind::FullHouse
                } else {
                    HandKind::Three
                }
            }
            2 => {
                if amounts.iter().take(12).filter(|x| **x == 2).count() == 2 {
                    HandKind::TwoPair
                } else {
                    HandKind::Pair
                }
            }
            1 => HandKind::HighCard,
            0 => HandKind::NoneMUST_BE_RANKED_UP,
            _ => unreachable!(),
        };
        for _ in 0..amounts[12] {
            kind = kind.rank_up_with_joker();
        }
        kind
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
    NoneMUST_BE_RANKED_UP,
    HighCard,
    Pair,
    TwoPair,
    Three,
    FullHouse,
    Four,
    Five,
}

impl HandKind {
    fn rank_up_with_joker(&self) -> Self {
        match self {
            HandKind::NoneMUST_BE_RANKED_UP => HandKind::HighCard,
            HandKind::HighCard => HandKind::Pair,
            HandKind::Pair => HandKind::Three,
            HandKind::TwoPair => HandKind::FullHouse,
            HandKind::Three => HandKind::Four,
            HandKind::FullHouse => HandKind::Four,
            HandKind::Four => HandKind::Five,
            HandKind::Five => HandKind::Five,
        }
    }
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
    J,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
    T,
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
            Card::T => 3,
            Card::_9 => 4,
            Card::_8 => 5,
            Card::_7 => 6,
            Card::_6 => 7,
            Card::_5 => 8,
            Card::_4 => 9,
            Card::_3 => 10,
            Card::_2 => 11,
            Card::J => 12,
        }
    }
}
