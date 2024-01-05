use std::{
    fs::File,
    io::{BufRead, BufReader},
};

const FACTOR: u128 = 999999;

fn main() {
    let mut galaxies = read_galaxies("input.txt");
    expand_galaxies(&mut galaxies);

    let mut sum = 0u128;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let g1 = galaxies[i];
            let g2 = galaxies[j];

            sum += g2.0.abs_diff(g1.0) + g2.1.abs_diff(g1.1);
        }
    }

    println!("{sum}");
}

fn expand_galaxies(galaxies: &mut Vec<(u128, u128)>) {
    galaxies.sort_by_key(|(x, _)| *x);

    let mut spaces = 0;
    let mut last = 0;
    for (x, _) in galaxies.iter_mut() {
        spaces += ((*x - last).max(1) - 1) * FACTOR;
        last = *x;
        *x += spaces;
    }

    galaxies.sort_by_key(|(_, y)| *y);

    let mut spaces = 0;
    let mut last = 0;
    for (_, y) in galaxies.iter_mut() {
        spaces += ((*y - last).max(1) - 1) * FACTOR;
        last = *y;
        *y += spaces;
    }
}

fn read_galaxies(path: &str) -> Vec<(u128, u128)> {
    let reader = BufReader::new(File::open(path).unwrap());

    let mut galaxies = vec![];

    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                galaxies.push((x as u128, y as u128));
            }
        }
    }

    galaxies
}
