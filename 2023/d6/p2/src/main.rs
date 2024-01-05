use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let mut lines = reader.lines();
    let times = get_ws_separated_numbers_after_one_non_numer(&mut lines);
    let distances = get_ws_separated_numbers_after_one_non_numer(&mut lines);

    let mut product = 1;
    for (time, distance) in times.into_iter().zip(distances) {
        product *= number_of_ways_to_win(time, distance);
    }

    println!("{product}");
}

fn number_of_ways_to_win(time: usize, record: usize) -> usize {
    let mut sum = 0;
    for hold in 0..=time {
        if hold * (time - hold) > record {
            sum += 1;
        }
    }
    sum
}

fn get_ws_separated_numbers_after_one_non_numer(
    lines: &mut std::io::Lines<BufReader<File>>,
) -> Vec<usize> {
    lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}
