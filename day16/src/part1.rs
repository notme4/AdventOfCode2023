use std::fs::read_to_string;
use std::path::Path;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Clone, Copy)]
enum Tile {
    Empty,
    VerticalSplitter,
    HorizontalSplitter,
    ForwardSlash, // '\'
    BackSlash,    // '/'
}

impl Tile {
    fn build(c: char) -> Option<Self> {
        match c {
            '.' => Some(Tile::Empty),
            '|' => Some(Tile::VerticalSplitter),
            '-' => Some(Tile::HorizontalSplitter),
            '\\' => Some(Tile::ForwardSlash),
            '/' => Some(Tile::BackSlash),
            _ => None,
        }
    }
}

type Map = Vec<Vec<(Tile, u64)>>;

fn get_new_direction(tile: &Tile, dir: Direction) -> Vec<Direction> {
    match tile {
        Tile::Empty => vec![dir],
        Tile::VerticalSplitter => match dir {
            Direction::Up | Direction::Down => vec![dir],
            Direction::Left | Direction::Right => vec![Direction::Up, Direction::Down],
        },
        Tile::HorizontalSplitter => match dir {
            Direction::Up | Direction::Down => vec![Direction::Left, Direction::Right],
            Direction::Left | Direction::Right => vec![dir],
        },
        Tile::ForwardSlash => match dir {
            Direction::Right => vec![Direction::Down],
            Direction::Left => vec![Direction::Up],
            Direction::Down => vec![Direction::Right],
            Direction::Up => vec![Direction::Left],
        },
        Tile::BackSlash => match dir {
            Direction::Right => vec![Direction::Up],
            Direction::Left => vec![Direction::Down],
            Direction::Down => vec![Direction::Left],
            Direction::Up => vec![Direction::Right],
        },
    }
}

fn dir_bit(dir: Direction) -> u64 {
    match dir {
        Direction::Left => 0b0001,
        Direction::Right => 0b0010,
        Direction::Up => 0b0100,
        Direction::Down => 0b1000,
    }
}

fn traverse(map: &mut Map, start_pos: (usize, usize), start_dir: Direction) {
    map[start_pos.0][start_pos.1].1 |= dir_bit(start_dir);
    let mut queue = vec![(start_pos, start_dir)];
    while !queue.is_empty() {
        let vector = queue.pop().unwrap();
        let pos = vector.0;
        let dir = vector.1;
        let new_pos = match dir {
            Direction::Left => {
                if pos.1 == 0 {
                    continue;
                }
                (pos.0, pos.1 - 1)
            }
            Direction::Right => {
                if pos.1 == map[0].len() - 1 {
                    continue;
                }
                (pos.0, pos.1 + 1)
            }
            Direction::Up => {
                if pos.0 == 0 {
                    continue;
                }
                (pos.0 - 1, pos.1)
            }
            Direction::Down => {
                if pos.0 == map.len() - 1 {
                    continue;
                }
                (pos.0 + 1, pos.1)
            }
        };
        let tile = &mut map[new_pos.0][new_pos.1].0;
        for new_dir in get_new_direction(tile, dir) {
            let i = &mut map[new_pos.0][new_pos.1].1;
            let bit = dir_bit(new_dir);
            if *i & bit == 0 {
                queue.push((new_pos, new_dir));
                *i |= bit;
            }
        }
        eprint_map(map);
        eprintln!("");
    }
}

fn eprint_map(map: &Map) {
    let map = map
        .iter()
        .map(|v| v.iter().map(|(_, i)| i).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    for v in map {
        for i in v {
            match i {
                0b0000 => eprint!(" "),
                0b0001 => eprint!("<"),
                0b0010 => eprint!(">"),
                0b0011 => eprint!("-"),
                0b0100 => eprint!("^"),
                0b0101 => eprint!("↖"),
                0b0110 => eprint!("↗"),
                0b0111 => eprint!("↑"),
                0b1000 => eprint!("v"),
                0b1001 => eprint!("↙"),
                0b1010 => eprint!("↘"),
                0b1011 => eprint!("↓"),
                0b1100 => eprint!("|"),
                0b1101 => eprint!("←"),
                0b1110 => eprint!("→"),
                0b1111 => eprint!("#"),
            }
        }
        eprintln!("");
    }
}

pub fn func(path: &Path) -> u64 {
    // let file_data = read_to_string(path).unwrap();
    let mut map: Map = read_to_string(path)
        .unwrap()
        .lines()
        .map(|s| s.chars().map(|c| (Tile::build(c).unwrap(), 0)).collect())
        .collect();
    traverse(&mut map, (0, 0), Direction::Right);
    eprint_map(&map);
    map.iter()
        .flatten()
        .filter(|(_, energy)| *energy != 0)
        .count()
        .try_into()
        .expect("small enough to fit in u64")
}
