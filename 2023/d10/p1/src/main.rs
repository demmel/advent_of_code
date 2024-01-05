use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let mut map: Vec<Vec<_>> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        map.push(line.chars().collect());
    }

    let (mut x, mut y, _) = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| (x, y, *c)))
        .find(|(_, _, c)| *c == 'S')
        .unwrap();

    let mut entered_from = None;
    let mut steps = 0;

    loop {
        let c = map[y][x];

        if c == 'S' {
            if entered_from.is_some() {
                break;
            }
            entered_from = [
                (0isize, -1isize, Direction::S),
                (0, 1, Direction::N),
                (-1, 0, Direction::E),
                (1, 0, Direction::W),
            ]
            .into_iter()
            .filter_map(|(dx, dy, dir)| {
                let py = y as isize + dy;
                if py < 0 {
                    None
                } else {
                    let px = x as isize + dx;
                    if px < 0 {
                        None
                    } else {
                        map.get(py as usize)
                            .map(|row| {
                                row.get(px as usize).map(|p| {
                                    connected_directions(*p)
                                        .into_iter()
                                        .filter(|d| *d == dir)
                                        .next()
                                })
                            })
                            .flatten()
                            .flatten()
                            .map(|d| Some(d))
                    }
                }
            })
            .next()
            .unwrap();
        } else {
            entered_from = Some(
                connected_directions(c)
                    .iter()
                    .find(|d| **d != entered_from.unwrap())
                    .unwrap()
                    .flip(),
            );
        }
        match entered_from.unwrap() {
            Direction::N => y += 1,
            Direction::S => y -= 1,
            Direction::W => x += 1,
            Direction::E => x -= 1,
        }
        steps += 1;
    }

    println!("{}", steps / 2);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    N,
    S,
    W,
    E,
}

impl Direction {
    fn flip(&self) -> Direction {
        match self {
            Direction::N => Self::S,
            Direction::S => Self::N,
            Direction::W => Self::E,
            Direction::E => Self::W,
        }
    }
}

fn connected_directions(c: char) -> [Direction; 2] {
    match c {
        '|' => [Direction::N, Direction::S],
        '-' => [Direction::W, Direction::E],
        'F' => [Direction::S, Direction::E],
        '7' => [Direction::W, Direction::S],
        'L' => [Direction::N, Direction::E],
        'J' => [Direction::N, Direction::W],
        _ => panic!("connected_directions: Inavlid pipe piece: {c}"),
    }
}
