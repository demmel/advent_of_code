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
                maps.insert(dst.to_string(), (src.to_string(), vec![]));
                phase = ParsePhase::MapItems(dst.to_string())
            }
            ParsePhase::MapItems(ref dst) => {
                let mut it = line.split(" ");
                let dst_start = it.next().unwrap().parse::<usize>().unwrap();
                let src_start = it.next().unwrap().parse::<usize>().unwrap();
                let n = it.next().unwrap().parse::<usize>().unwrap();
                maps.get_mut(dst).unwrap().1.push((src_start, dst_start, n));
            }
        }
    }

    let items: Vec<_> = seeds.chunks(2).map(|x| (x[0], x[1])).collect();

    let mut location = 0;
    'outer: loop {
        let seed = get_seed_for_location(&maps, location);
        for (start, len) in items.iter() {
            if seed >= *start && seed < start + len {
                break 'outer;
            }
        }
        location += 1;
    }

    println!("{location}");
}

fn get_seed_for_location(
    maps: &HashMap<String, (String, Vec<(usize, usize, usize)>)>,
    location: usize,
) -> usize {
    let mut item_type = "location";
    let mut item = location;

    while item_type != "seed" {
        let map = maps.get(item_type).unwrap();

        for (src_start, dst_start, len) in map.1.iter() {
            if item >= *dst_start && item < *dst_start + *len {
                item = item - dst_start + src_start;
                break;
            }
        }

        item_type = &map.0;
    }

    item as usize
}
