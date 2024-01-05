use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());

    let mut lines = reader.lines();
    let directions = lines.next().unwrap().unwrap();

    lines.next();

    let mut map: HashMap<String, (String, String)> = HashMap::new();
    for line in lines {
        let line = line.unwrap();
        let (node, rest) = line.split_once("=").unwrap();
        let (left, right) = rest.split_once(", ").unwrap();

        map.insert(
            node.trim().to_string(),
            (
                left.split("(").nth(1).unwrap().to_string(),
                right.split(")").next().unwrap().to_string(),
            ),
        );
    }

    let mut node = "AAA";
    let mut steps = 0;
    let mut direction_it = directions.chars().cycle();

    while node != "ZZZ" {
        let node_map = map.get(node).unwrap();
        node = match direction_it.next().unwrap() {
            'L' => &node_map.0,
            'R' => &node_map.1,
            _ => unreachable!(),
        };
        steps += 1;
    }

    println!("{steps}");
}
