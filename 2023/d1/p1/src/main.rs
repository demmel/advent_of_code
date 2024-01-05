use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let d1 = line.chars().find(|c| c.is_numeric()).unwrap();
        let d2 = line.chars().rev().find(|c| c.is_numeric()).unwrap();
        let num = format!("{d1}{d2}").parse::<u32>().unwrap();
        sum += num;
    }
    println!("{sum}");
}
