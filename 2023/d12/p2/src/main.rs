use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut sum = 0;

    let reader = BufReader::new(File::open("input.txt").unwrap());

    for line in reader.lines() {
        let line = line.unwrap();
        println!("{line}");

        let (record, checksums) = line.split_once(" ").unwrap();
        let checksums: Vec<_> = checksums
            .split(",")
            .map(|c| c.parse::<usize>().unwrap())
            .collect();

        let record = [record, record, record, record, record].join("?");
        let checksums = checksums.repeat(5);

        let mut cache = HashMap::new();
        sum += count_arrangements(&mut cache, &record, &checksums);
    }

    println!("{sum}");
}

fn count_arrangements<'a>(
    cache: &mut HashMap<(&'a str, &'a [usize]), usize>,
    record: &'a str,
    checksums: &'a [usize],
) -> usize {
    if cache.contains_key(&(record, checksums)) {
        return cache[&(record, checksums)];
    }

    if checksums.is_empty() {
        return if record.chars().filter(|c| *c == '#').count() == 0 {
            1
        } else {
            0
        };
    }

    let checksum: &usize = checksums.first().unwrap();
    let mut sum = 0;

    'outer: for i in record
        .chars()
        .enumerate()
        .filter(|(_, c)| *c == '?' || *c == '#')
        .filter(|(i, _)| record.chars().take(*i).all(|c| c != '#'))
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
            sum += count_arrangements(cache, &record[(i + checksum + 1)..], &checksums[1..]);
        } else {
            sum += count_arrangements(cache, "", &checksums[1..]);
        }
    }

    cache.insert((record, checksums), sum);

    sum
}
