use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let mut sum = 0;
    let mut prev_numbers: Vec<(isize, String)> = vec![];
    let mut prev_symbols = vec![];

    let reader = BufReader::new(File::open("input.txt").unwrap());
    for line in reader.lines() {
        let line = line.unwrap();

        let mut numbers = vec![];
        let mut symbols = vec![];
        let mut number = None;

        for (i, c) in line.chars().enumerate() {
            match c {
                '0'..='9' => match number {
                    None => number = Some((i as isize, c.to_string())),
                    Some((_, ref mut s)) => s.push(c),
                },
                c => {
                    if let Some(n) = number {
                        numbers.push(n);
                        number = None;
                    }
                    if c != '.' {
                        symbols.push(i);
                    }
                }
            }
        }
        if let Some(n) = number {
            numbers.push(n);
        }

        numbers.retain_mut(|(p, n)| {
            let start = *p - 1;
            let end = *p + n.len() as isize;
            for sp in symbols.iter().chain(prev_symbols.iter()) {
                if start <= *sp as isize && *sp as isize <= end {
                    sum += n.parse::<u32>().unwrap();
                    return false;
                }
            }
            true
        });

        for (p, n) in prev_numbers {
            let start = p - 1;
            let end = p + n.len() as isize;
            for sp in symbols.iter() {
                if start <= *sp as isize && *sp as isize <= end {
                    sum += n.parse::<u32>().unwrap();
                }
            }
        }

        prev_numbers = numbers;
        prev_symbols = symbols;
    }

    println!("{sum}");
}
