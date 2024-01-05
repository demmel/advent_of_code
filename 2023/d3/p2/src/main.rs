use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut sum = 0;

    let mut numbers: Vec<(isize, isize, String)> = vec![];
    let mut gears = vec![];

    let reader = BufReader::new(File::open("input.txt").unwrap());
    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();

        let mut number = None;

        for (x, c) in line.chars().enumerate() {
            match c {
                '0'..='9' => match number {
                    None => number = Some((x as isize, y as isize, c.to_string())),
                    Some((_, _, ref mut s)) => s.push(c),
                },
                c => {
                    if let Some(n) = number {
                        numbers.push(n);
                        number = None;
                    }
                    if c == '*' {
                        gears.push((x as isize, y as isize));
                    }
                }
            }
        }
        if let Some(n) = number {
            numbers.push(n);
        }
    }

    for (gx, gy) in gears {
        let mut found = 0;
        let mut ratio = 1;
        for (nx, ny, n) in &numbers {
            if gy >= *ny - 1 && gy <= *ny + 1 && gx >= *nx - 1 && gx <= *nx + n.len() as isize {
                found += 1;
                ratio *= n.parse::<u32>().unwrap();
            }
        }
        if found == 2 {
            sum += ratio;
        }
    }

    println!("{sum}");
}
