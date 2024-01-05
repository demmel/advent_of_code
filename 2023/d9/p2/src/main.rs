use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let histories = read_histories();
    let mut sum = 0;
    for history in histories {
        sum += prev_in_seq(&history);
    }
    println!("{sum}");
}

fn prev_in_seq(history: &[i32]) -> i32 {
    let diff_seq: Vec<_> = history.windows(2).map(|w| w[1] - w[0]).collect();
    let ret = history.first().unwrap()
        - if diff_seq.iter().all(|x| *x == 0) {
            0
        } else {
            prev_in_seq(&diff_seq)
        };
    ret
}

fn read_histories() -> Vec<Vec<i32>> {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let mut histories = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        histories.push(parse_history(&line));
    }
    histories
}

fn parse_history(s: &str) -> Vec<i32> {
    s.split(" ").map(|s| s.parse::<i32>().unwrap()).collect()
}
