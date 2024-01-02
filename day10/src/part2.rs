use std::fs::read_to_string;
use std::path::Path;

///   |
///   -
///   L
///   J
///   7
///   F
///   .
///
/// v | v
/// ^ | ^
/// > | .
/// < | .
/// v L >
/// ^ L .
/// > L .
/// < L ^
/// v J <
/// ^ J .
/// > J ^
/// < J .
/// v F .
/// ^ F >
/// > F .
/// < F v
/// v 7 .
/// ^ 7 <
/// > 7 v
/// < 7 .
/// v - .
/// ^ - .
/// > - >
/// < - <
/// v . .
/// ^ . .
/// > . .
/// < . .
///
type PipeMap = Vec<Vec<char>>;
type Pos = (usize, usize);

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn move_down(pos: Pos, pipe_map: &PipeMap) -> Option<(Direction, Pos)> {
    let pos = (pos.0 + 1, pos.1);
    let pipe = pipe_map.get(pos.0)?.get(pos.1);
    match pipe? {
        '|' => return Some((Direction::Down, pos)),
        '-' => return None,
        'L' => return Some((Direction::Right, pos)),
        'J' => return Some((Direction::Left, pos)),
        'F' => return None,
        '7' => return None,
        '.' => return None,
        _ => return None,
    }
}

fn move_up(pos: Pos, pipe_map: &PipeMap) -> Option<(Direction, Pos)> {
    if pos.0 == 0 {
        return None;
    }
    let pos = (pos.0 - 1, pos.1);
    let pipe = pipe_map.get(pos.0)?.get(pos.1);
    match pipe? {
        '|' => return Some((Direction::Up, pos)),
        '-' => return None,
        'L' => return None,
        'J' => return None,
        'F' => return Some((Direction::Right, pos)),
        '7' => return Some((Direction::Left, pos)),
        '.' => return None,
        _ => return None,
    }
}

fn move_left(pos: Pos, pipe_map: &PipeMap) -> Option<(Direction, Pos)> {
    if pos.1 == 0 {
        return None;
    }
    let pos = (pos.0, pos.1 - 1);
    let pipe = pipe_map.get(pos.0)?.get(pos.1);
    match pipe? {
        '|' => return None,
        '-' => return Some((Direction::Left, pos)),
        'L' => return Some((Direction::Up, pos)),
        'J' => return None,
        'F' => return Some((Direction::Down, pos)),
        '7' => return None,
        '.' => return None,
        _ => return None,
    }
}

fn move_right(pos: Pos, pipe_map: &PipeMap) -> Option<(Direction, Pos)> {
    let pos = (pos.0, pos.1 + 1);
    let pipe = pipe_map.get(pos.0)?.get(pos.1);
    match pipe? {
        '|' => return None,
        '-' => return Some((Direction::Right, pos)),
        'L' => return None,
        'J' => return Some((Direction::Up, pos)),
        'F' => return None,
        '7' => return Some((Direction::Down, pos)),
        '.' => return None,
        _ => return None,
    }
}

fn move_once(dir: Direction, pos: Pos, pipe_map: &PipeMap) -> Option<(Direction, Pos)> {
    match dir {
        Direction::Up => return move_up(pos, pipe_map),
        Direction::Down => return move_down(pos, pipe_map),
        Direction::Left => return move_left(pos, pipe_map),
        Direction::Right => return move_right(pos, pipe_map),
    }
}

fn find_loop(start: Pos, pipe_map: &PipeMap) -> Direction {
    let mut positions = vec![
        (Direction::Up, start),
        (Direction::Down, start),
        (Direction::Left, start),
        (Direction::Right, start),
    ];
    let mut result: u64 = 1;
    loop {
        eprintln!("{:?}", positions);
        if result as usize >= pipe_map.len() * pipe_map[0].len() {
            panic!("should have ended by now");
        }
        if positions.len() < 2 {
            panic!("not enough positions");
        }
        let mut i = 0;
        while i < positions.len() {
            let pos = &positions[i];
            let new_pos = move_once(pos.0, pos.1, pipe_map);
            if new_pos.is_none() {
                positions[i] = (Direction::Up, (0, 0));
                //eprintln!("removed {i}");
                i += 1;
                continue;
            }
            let new_pos = new_pos.unwrap();
            if positions.iter().any(|val| val.1 == new_pos.1) {
                match i {
                    0 => return Direction::Up,
                    1 => return Direction::Down,
                    2 => return Direction::Left,
                    3 => return Direction::Right,
                    _ => panic!("invalid position index"),
                }
            }
            positions[i] = new_pos;

            i += 1;
        }
        result += 1;
    }
}

pub fn func(path: &Path) -> u64 {
    let mut pipe_map = read_to_string(path)
        .unwrap()
        .lines()
        .map(|val| val.chars().collect::<Vec<char>>())
        .collect::<PipeMap>();
    let i = pipe_map
        .iter()
        .position(|val| val.iter().any(|c| *c == 'S'))
        .unwrap();
    let j = pipe_map[i].iter().position(|c| *c == 'S').unwrap();
    let mut dir_pos = Some((find_loop((i, j), &pipe_map), (i, j)));
    while dir_pos.is_some() {
        let pos = dir_pos.unwrap();
        eprintln!("pos: {:?}", pos);
        match pipe_map[pos.1 .0][pos.1 .1] {
            '-' => pipe_map[pos.1 .0][pos.1 .1] = '━',
            '|' => pipe_map[pos.1 .0][pos.1 .1] = '┃',
            'J' => pipe_map[pos.1 .0][pos.1 .1] = '┛',
            'L' => pipe_map[pos.1 .0][pos.1 .1] = '┗',
            'F' => pipe_map[pos.1 .0][pos.1 .1] = '┏',
            '7' => pipe_map[pos.1 .0][pos.1 .1] = '┓',
            _ => pipe_map[pos.1 .0][pos.1 .1] = '*',
        }
        dir_pos = move_once(pos.0, pos.1, &pipe_map);
    }
    let mut result = 0;
    for line in pipe_map {
        let mut inside = false;
        let mut last_lpipe = ' ';
        for pipe in line {
            match pipe {
                '┃' => {
                    eprint!("{pipe}");
                    inside = !inside;
                }
                '┛' => {
                    eprint!("{pipe}");
                    match last_lpipe {
                        '┏' => {
                            inside = !inside;
                            last_lpipe = ' ';
                        }
                        ' ' => last_lpipe = '┛',
                        _ => last_lpipe = ' ',
                    }
                }
                '┗' => {
                    eprint!("{pipe}");
                    match last_lpipe {
                        '┓' => {
                            inside = !inside;
                            last_lpipe = ' ';
                        }
                        ' ' => last_lpipe = '┗',
                        _ => last_lpipe = ' ',
                    }
                }
                '┏' => {
                    eprint!("{pipe}");
                    match last_lpipe {
                        '┛' => {
                            inside = !inside;
                            last_lpipe = ' ';
                        }
                        ' ' => last_lpipe = '┏',
                        _ => last_lpipe = ' ',
                    }
                }
                '┓' | '*' => {
                    eprint!("{pipe}");
                    match last_lpipe {
                        '┗' => {
                            inside = !inside;
                            last_lpipe = ' ';
                        }
                        ' ' => last_lpipe = '┓',
                        _ => last_lpipe = ' ',
                    }
                }
                '━' => {
                    eprint!("{pipe}");
                }
                _ => {
                    if inside {
                        eprint!(".");
                        result += 1
                    } else {
                        eprint!(" ");
                    }
                }
            }
        }
        eprintln!("");
    }
    return result;
}
