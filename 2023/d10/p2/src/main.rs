use std::{
    collections::HashSet,
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
    vec,
};

fn main() {
    let (sx, sy, map) = read_map("input.txt");
    let pipes = find_loop_pipes(&map, sx, sy);

    let mut sections = vec![vec![PipeFace::Out; map[0].len()]; map.len()];

    let (start_i, &(sx, sy)) = pipes
        .iter()
        .enumerate()
        .min_by_key(|(_, coord)| (coord.1, coord.0))
        .unwrap();
    let mut prev = (sx, sy, map[sy][sx]);

    for row in 0..map.len() {
        for cell in 0..map[0].len() {
            if pipes.contains(&(cell, row)) {
                print!("{}", map[row][cell]);
            } else {
                print!(".");
            }
        }
        println!();
    }

    sections[sy][sx] = PipeFace::SE;
    for &(x, y) in pipes[(start_i + 1)..].iter().chain(pipes[..start_i].iter()) {
        let c = map[y][x];
        let ps = sections[prev.1][prev.0].clone();

        sections[y][x] = match c {
            '|' => ps.horizontal(),
            '-' => ps.vertical(),
            'L' => match prev.2 {
                '|' | '-' | '7' => ps.ne_sw(),
                'J' => ps.vertical().ne_sw(),
                'F' => ps.horizontal().ne_sw(),
                _ => unreachable!(),
            },
            '7' => match prev.2 {
                '|' | '-' | 'L' => ps.ne_sw(),
                'J' => ps.horizontal().ne_sw(),
                'F' => ps.vertical().ne_sw(),
                _ => unreachable!(),
            },
            'J' => match prev.2 {
                '|' | '-' | 'F' => ps.nw_se(),
                'L' => ps.vertical().nw_se(),
                '7' => ps.horizontal().nw_se(),
                _ => unreachable!(),
            },
            'F' => match prev.2 {
                '|' | '-' | 'J' => ps.nw_se(),
                'L' => ps.horizontal().nw_se(),
                '7' => ps.vertical().nw_se(),
                _ => unreachable!(),
            },
            _ => unreachable!(),
        };

        // println!("{} ({}) -> {} ({})", prev.2, ps, c, sections[y][x]);

        prev = (x, y, c);
    }

    for y in 1..sections.len() {
        for x in 1..sections[0].len() {
            if sections[y][x] == PipeFace::Out {
                sections[y][x] = match (
                    &sections[y - 1][x - 1],
                    &sections[y - 1][x],
                    &sections[y][x - 1],
                ) {
                    (PipeFace::SE, _, _) => PipeFace::In,
                    (PipeFace::NW, _, _) => PipeFace::Out,
                    (_, PipeFace::S | PipeFace::SW | PipeFace::SE, _) => PipeFace::In,
                    (_, PipeFace::N | PipeFace::NW | PipeFace::NE, _) => PipeFace::Out,
                    (_, _, PipeFace::E | PipeFace::SE | PipeFace::NE) => PipeFace::In,
                    (_, _, PipeFace::W | PipeFace::SW | PipeFace::NW) => PipeFace::Out,
                    (PipeFace::In, _, _) => PipeFace::In,
                    (_, PipeFace::In, _) => PipeFace::In,
                    (_, _, PipeFace::In) => PipeFace::In,
                    (PipeFace::Out, _, _) => PipeFace::Out,
                    (_, PipeFace::Out, _) => PipeFace::Out,
                    (_, _, PipeFace::Out) => PipeFace::Out,
                    p => {
                        for row in &sections {
                            for cell in row {
                                print!("{cell}");
                            }
                            println!();
                        }
                        println!("({x}, {y})");
                        panic!("{p:?}")
                    }
                }
            }
        }
    }

    let mut count = 0;
    for row in sections {
        for cell in row {
            if cell == PipeFace::In {
                count += 1;
            }
            print!("{cell}");
        }
        println!();
    }
    println!("{count}");
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PipeFace {
    Out,
    In,
    N,
    S,
    W,
    E,
    NW,
    SW,
    NE,
    SE,
}

impl PipeFace {
    fn flip(&self) -> PipeFace {
        match self {
            PipeFace::Out => PipeFace::In,
            PipeFace::In => PipeFace::Out,
            PipeFace::N => Self::S,
            PipeFace::S => PipeFace::N,
            PipeFace::W => PipeFace::E,
            PipeFace::E => PipeFace::W,
            PipeFace::NW => Self::SE,
            PipeFace::SW => Self::NE,
            PipeFace::NE => Self::SW,
            PipeFace::SE => PipeFace::NW,
        }
    }

    fn vertical(&self) -> PipeFace {
        match self {
            PipeFace::N => Self::N,
            PipeFace::S => Self::S,
            PipeFace::NW => Self::N,
            PipeFace::SW => Self::S,
            PipeFace::NE => Self::N,
            PipeFace::SE => Self::S,
            _ => unimplemented!(),
        }
    }

    fn horizontal(&self) -> PipeFace {
        match self {
            PipeFace::W => Self::W,
            PipeFace::E => Self::E,
            PipeFace::NW => PipeFace::W,
            PipeFace::SW => Self::W,
            PipeFace::NE => Self::E,
            PipeFace::SE => Self::E,
            _ => unimplemented!(),
        }
    }

    fn nw_se(&self) -> PipeFace {
        match self {
            PipeFace::W => Self::NW,
            PipeFace::E => Self::SE,
            PipeFace::NW => Self::NW,
            PipeFace::S => Self::SE,
            PipeFace::N => Self::NW,
            PipeFace::SE => Self::SE,
            _ => unimplemented!(),
        }
    }

    fn ne_sw(&self) -> PipeFace {
        match self {
            PipeFace::W => Self::SW,
            PipeFace::E => Self::NE,
            PipeFace::SW => Self::SW,
            PipeFace::S => Self::SW,
            PipeFace::N => Self::NE,
            PipeFace::NE => Self::NE,
            _ => unimplemented!(),
        }
    }
}

impl Display for PipeFace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                PipeFace::Out => '-',
                PipeFace::In => 'X',
                // PipeFace::N => '^',
                // PipeFace::S => 'v',
                // PipeFace::W => '<',
                // PipeFace::E => '>',
                // PipeFace::NW => 'F',
                // PipeFace::SW => 'L',
                // PipeFace::NE => '7',
                // PipeFace::SE => 'J',
                _ => '-',
            }
        )
    }
}

fn read_map(path: &str) -> (usize, usize, Vec<Vec<char>>) {
    let reader = BufReader::new(File::open(path).unwrap());
    let mut map: Vec<Vec<_>> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        map.push(line.chars().collect());
    }

    let (x, y, _) = map
        .iter()
        .enumerate()
        .flat_map(|(y, row)| row.iter().enumerate().map(move |(x, c)| (x, y, *c)))
        .find(|(_, _, c)| *c == 'S')
        .unwrap();

    let start_symbol = ['|', 'F', '-', '7', 'J', 'L']
        .into_iter()
        .filter(|c| {
            let ds = connected_directions(*c);
            let ds = if let Some(ds) = ds {
                ds
            } else {
                return false;
            };
            ds.iter().all(|d| {
                (match d {
                    Direction::N => {
                        if y == 0 {
                            return false;
                        } else {
                            connected_directions(map[y - 1][x])
                        }
                    }
                    Direction::S => {
                        if y == map.len() - 1 {
                            return false;
                        } else {
                            connected_directions(map[y + 1][x])
                        }
                    }
                    Direction::W => {
                        if x == 0 {
                            return false;
                        } else {
                            connected_directions(map[y][x - 1])
                        }
                    }
                    Direction::E => {
                        if x == map[0].len() - 1 {
                            return false;
                        } else {
                            connected_directions(map[y][x + 1])
                        }
                    }
                })
                .map(|ods| ods.iter().any(|od| *od == d.flip()))
                .unwrap_or(false)
            })
        })
        .next()
        .unwrap();

    map[y][x] = start_symbol;

    (x, y, map)
}

fn find_loop_pipes(map: &Vec<Vec<char>>, sx: usize, sy: usize) -> Vec<(usize, usize)> {
    let (mut x, mut y) = (sx, sy);

    let mut entered_from = *connected_directions(map[sy][sx]).unwrap().first().unwrap();
    let mut has_loop_pipe = HashSet::new();
    let mut order = vec![];

    while !has_loop_pipe.contains(&(x, y)) {
        has_loop_pipe.insert((x, y));
        order.push((x, y));
        let c = map[y][x];

        entered_from = connected_directions(c)
            .unwrap()
            .iter()
            .find(|d| **d != entered_from)
            .unwrap()
            .flip();

        match entered_from {
            Direction::N => y += 1,
            Direction::S => y -= 1,
            Direction::W => x += 1,
            Direction::E => x -= 1,
        }
    }

    order
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

fn connected_directions(c: char) -> Option<[Direction; 2]> {
    Some(match c {
        '|' => [Direction::N, Direction::S],
        '-' => [Direction::W, Direction::E],
        'F' => [Direction::S, Direction::E],
        '7' => [Direction::W, Direction::S],
        'L' => [Direction::N, Direction::E],
        'J' => [Direction::N, Direction::W],
        '.' => return None,
        _ => panic!("connected_directions: Inavlid pipe piece: {c}"),
    })
}
