use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

enum ParsePhase {
    Seeds,
    MapHeader,
    MapItems(String),
}

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let mut seeds: Vec<usize> = vec![];
    let mut maps: HashMap<String, (String, Vec<(usize, usize, usize)>)> = HashMap::new();
    let mut phase = ParsePhase::Seeds;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            phase = ParsePhase::MapHeader;
            continue;
        }
        match phase {
            ParsePhase::Seeds => {
                seeds = line
                    .split(":")
                    .nth(1)
                    .unwrap()
                    .trim()
                    .split(" ")
                    .map(|s| s.parse::<usize>().unwrap())
                    .collect()
            }
            ParsePhase::MapHeader => {
                let (src, dst) = line.split(" ").next().unwrap().split_once("-to-").unwrap();
                maps.insert(src.to_string(), (dst.to_string(), vec![]));
                phase = ParsePhase::MapItems(src.to_string())
            }
            ParsePhase::MapItems(ref src) => {
                let mut it = line.split(" ");
                let dst_start = it.next().unwrap().parse::<usize>().unwrap();
                let src_start = it.next().unwrap().parse::<usize>().unwrap();
                let n = it.next().unwrap().parse::<usize>().unwrap();
                maps.get_mut(src).unwrap().1.push((src_start, dst_start, n));
            }
        }
    }

    let mut item_type = "seed";
    let mut items = seeds;

    while item_type != "location" {
        let map = maps.get(item_type).unwrap();

        for item in items.iter_mut() {
            for (src_start, dst_start, len) in map.1.iter() {
                if *item >= *src_start && *item < *src_start + *len {
                    *item = *item - src_start + dst_start;
                    break;
                }
            }
        }

        item_type = &map.0;
    }

    println!("{}", items.iter().min().unwrap());
}
