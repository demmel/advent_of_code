use std::{
    collections::HashMap,
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let (directions, map) = read_directions_and_map();

    let nodes: Vec<_> = map.keys().filter(|n| n.ends_with('A')).map(|s| s).collect();
    let mut cycle_starts = vec![];
    let mut cycle_lens = vec![];
    let mut z_indexes = vec![];

    for mut node in nodes {
        let mut step = 0;
        let mut direction_it = directions.chars().enumerate().cycle();
        let mut direction = direction_it.next().unwrap();
        let mut seen = HashMap::new();

        while !seen.contains_key(&(node, direction.0)) {
            seen.insert((node, direction.0), step);

            if node.ends_with('Z') {
                z_indexes.push(step)
            }

            let node_map = map.get(node).unwrap();
            node = match direction.1 {
                'L' => &node_map.0,
                'R' => &node_map.1,
                _ => unreachable!(),
            };
            direction = direction_it.next().unwrap();
            step += 1;
        }

        let cycle_start = *seen.get(&(node, direction.0)).unwrap();
        cycle_starts.push(cycle_start);
        cycle_lens.push(step - cycle_start);
    }

    println!("{}", lcm(&cycle_lens));
}

fn lcm(nums: &[usize]) -> usize {
    let mut nums: Vec<_> = nums.iter().cloned().collect();
    let mut gcd = 1;

    'outer: loop {
        let min = nums.iter().min().unwrap();
        for f in 2..=*min {
            if nums.iter().all(|n| n % f == 0) {
                gcd *= f;
                for n in &mut nums {
                    *n /= f;
                }
                continue 'outer;
            }
        }
        break;
    }

    nums.iter().product::<usize>() * gcd
}

fn read_directions_and_map() -> (String, HashMap<String, (String, String)>) {
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

    (directions, map)
}
