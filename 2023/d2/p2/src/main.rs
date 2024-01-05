use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let game = parse_game(&line);

        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for outcome in game.outcomes {
            red = red.max(outcome.red);
            green = green.max(outcome.green);
            blue = blue.max(outcome.blue);
        }
        sum += red * green * blue;
    }
    println!("{sum}");
}

struct Game {
    id: u32,
    outcomes: Vec<Outcome>,
}

struct Outcome {
    red: u32,
    green: u32,
    blue: u32,
}

fn parse_game(s: &str) -> Game {
    let mut iter = s.split(":");
    let id = iter
        .next()
        .unwrap()
        .split(" ")
        .nth(1)
        .unwrap()
        .parse::<u32>()
        .unwrap();
    let outcomes = iter
        .next()
        .unwrap()
        .split(";")
        .map(|s| parse_outcome(s.trim()))
        .collect();
    Game { id, outcomes }
}

fn parse_outcome(s: &str) -> Outcome {
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;
    for cube_count_and_color in s.split(",") {
        let mut iter = cube_count_and_color.trim().split(" ");
        let count = iter.next().unwrap().parse::<u32>().unwrap();
        match iter.next().unwrap().trim() {
            "red" => {
                red = count;
            }
            "blue" => {
                blue = count;
            }
            "green" => {
                green = count;
            }
            _ => unreachable!(),
        }
    }
    Outcome { red, green, blue }
}
