use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut cards = vec![];
    let reader = BufReader::new(File::open("input.txt").unwrap());
    for line in reader.lines() {
        let line = line.unwrap();
        let card = parse_card(&line);
        cards.push(card)
    }
    let mut n_cards = vec![1; cards.len()];
    for (i, card) in cards.iter().enumerate() {
        let matches = card.winning.intersection(&card.have).count();
        for j in (i + 1)..(i + 1 + matches) {
            n_cards[j] += n_cards[i];
        }
    }
    println!("{}", n_cards.iter().sum::<u32>());
}

struct Card {
    winning: HashSet<u32>,
    have: HashSet<u32>,
}

fn parse_card(s: &str) -> Card {
    let numbers = s.split(":").nth(1).unwrap();
    let (winning, have) = numbers.split_once("|").unwrap();
    let winning = winning
        .trim()
        .split(" ")
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    let have = have
        .trim()
        .split(" ")
        .filter(|s| s.len() > 0)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    Card { winning, have }
}
