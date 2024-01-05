use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut sum = 0;

    let reader = BufReader::new(File::open("input.txt").unwrap());

    for line in reader.lines() {
        let line = line.unwrap();

        let (record, checksums) = line.split_once(" ").unwrap();
        let checksums: Vec<_> = checksums
            .split(",")
            .map(|c| c.parse::<usize>().unwrap())
            .collect();

        sum += count_arrangements(record, &checksums);
    }

    println!("{sum}");
}

fn count_arrangements(record: &str, checksums: &[usize]) -> usize {
    if checksums.is_empty() {
        return if record.chars().filter(|c| *c == '#').count() == 0 {
            1
        } else {
            0
        };
    }

    let checksum = checksums.first().unwrap();
    let mut sum = 0;

    'outer: for i in record
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '?' || *c == '#')
        .map(|(i, _)| i)
    {
        let mut remaining = *checksum;
        let mut it = record.chars().skip(i);

        while remaining > 0 {
            let c = if let Some(c) = it.next() {
                c
            } else {
                continue 'outer;
            };

            if c == '.' {
                continue 'outer;
            }

            remaining -= 1;
        }

        if let Some(c) = it.next() {
            if c == '#' {
                continue 'outer;
            }
        }

        sum += count_arrangements(&record[(i + checksum)..], &checksums[1..]);
    }

    println!("{record} {checksums:?} -> {sum}");

    sum
}
