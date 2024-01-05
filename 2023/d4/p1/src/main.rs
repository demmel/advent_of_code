use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut sum = 0;
    let reader = BufReader::new(File::open("input.txt").unwrap());
    for line in reader.lines() {
        let line = line.unwrap();
        let card = parse_card(&line);
        let matches = card.winning.intersection(&card.have).count();
        sum += if matches > 0 {
            2u32.pow(matches as u32 - 1)
        } else {
            0
        };
    }
    println!("{sum}");
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
