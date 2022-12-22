use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::collections::HashMap;
use std::iter;

const RENDER: bool = true;

type Pos = (u8, u8);
#[derive(Copy, Clone, Debug, PartialEq)]
enum Tile {Open, Wall}
type Map = HashMap<Pos, Tile>;
#[derive(Copy, Clone, Debug, PartialEq)]
enum Turn {L = -1, R = 1}
type Move = (u8, Turn);
type Path = Vec<Move>;
#[derive(Copy, Clone, Debug, PartialEq)]
enum Facing {
    Right = 0,
    Down = 1,
    Left = 2,
    Up = 3,
}

fn parse(input: &str) -> (Map, Path) {
    let mut it = input.trim_end().split("\n\n");
    let map = it.next().unwrap()
        .split('\n')
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| {
                    let pos = (x as u8 + 1, y as u8 + 1);
                    match c {
                        '.' => Some((pos, Tile::Open)),
                        '#' => Some((pos, Tile::Wall)),
                        ' ' => None,
                        _ => panic!("Unexpected map char"),
                    }
                })
        }).collect();
    let path = it.next().unwrap()
        .split_inclusive(&['R', 'L'][..])
        .map(|m| {
            let turn = match &m[m.len() - 1..m.len()] {
                "L" => Turn::L,
                "R" => Turn::R,
                _ => return (m.parse::<u8>().unwrap(), Turn::R),
            };
            let forward = m[0..m.len() - 1].parse::<u8>().unwrap();
            (forward, turn)
        })
        .chain(iter::once((0, Turn::L)))
        .collect();
    (map, path)

}

fn facing_char(f: Facing) -> char {
    match f {
        Facing::Right => '>',
        Facing::Down => 'v',
        Facing::Left => '<',
        Facing::Up => '^',
    }
}

fn render(map: &Map, pos: Pos, f: Facing) {
    if !RENDER { return; }

    if map.get(&pos) != Some(&Tile::Open) {
        panic!("Invalid position");
    }

    let y1 = if pos.1 < 25 { 0 } else { pos.1 - 25 };
    let y2 = y1 + 50;
    print!("\x1b[{}A", y2 - y1 + 1);

    let render: String = (y1..y2).map(|y| {
        (1..=150).map(|x| {
            let p = (x, y);
            if pos == p {
                format!("\x1b[31;1;1m{}\x1b[0m", facing_char(f))
            } else {
                match map.get(&p) {
                    Some(Tile::Open) => ".",
                    Some(Tile::Wall) => "\x1b[33;1;1m#\x1b[0m",
                    None => " ",
                }.to_string()
            }
        }).collect::<String>()
    })
    .fold(String::new(), |a, b| a + &b + "\n");
    print!("{}", render);

    let mut stdin = io::stdin();
    let _ = stdin.read(&mut [0u8]).unwrap();
}

fn solution(map: &Map, path: &Path, wrap_fn: &mut dyn FnMut(Pos, Facing) -> (Pos, Facing)) -> usize {
    let mut pos = map.keys().copied().min_by_key(|(x, y)| (*y, *x)).unwrap();
    let mut f = Facing::Right;
    for (fw, turn) in path.iter().copied() {
        for _i in 0..fw {
            let p2 = match f {
                Facing::Right => (pos.0 + 1, pos.1),
                Facing::Down => (pos.0, pos.1 + 1),
                Facing::Left => (pos.0 - 1, pos.1),
                Facing::Up => (pos.0, pos.1 - 1),
            };
            match map.get(&p2) {
                Some(Tile::Open) => pos = p2,
                Some(Tile::Wall) => break,
                None => {
                    let inspect = false;
                    let inspect_pos = pos;
                    if inspect {
                        println!("1:{},{} {} {} {}", pos.0, pos.1, facing_char(f), fw, if turn == Turn::L {'L'} else {'R'});
                        render(&map, pos, f);
                    }
                    let (p3, f2) = wrap_fn(pos, f);
                    if map.get(&p3) == Some(&Tile::Wall) { break; }
                    pos = p3;
                    f = f2;
                    if inspect {
                        println!("2:{},{} -> {},{} {} {} {}", inspect_pos.0, inspect_pos.1, pos.0, pos.1, facing_char(f), fw, if turn == Turn::L {'L'} else {'R'});
                        render(&map, pos, f);
                    }
                }
            }
        }
        f = match (4 + f as i8 + turn as i8) % 4 {
            0 => Facing::Right,
            1 => Facing::Down,
            2 => Facing::Left,
            3 => Facing::Up,
            _ => panic!("Bad value"),
        };
    }
    1000 * pos.1 as usize + 4 * pos.0 as usize + f as usize
}

fn solution1(input: &str) -> usize {
    let (map, path) = parse(input);
    solution(&map, &path, &mut |p2: Pos, f: Facing| -> (Pos, Facing) {
        let it = map.keys().copied();
        let p3 = match f {
            Facing::Right =>
                it.filter(|p| p.1 == p2.1).min_by_key(|p| p.0).unwrap(),
            Facing::Down =>
                it.filter(|p| p.0 == p2.0).min_by_key(|p| p.1).unwrap(),
            Facing::Left =>
                it.filter(|p| p.1 == p2.1).max_by_key(|p| p.0).unwrap(),
            Facing::Up =>
                it.filter(|p| p.0 == p2.0).max_by_key(|p| p.1).unwrap(),
        };
        (p3, f)
    })
}

fn solution2(input: &str) -> usize {
    let (map, path) = parse(input);
    let side = map.keys().flat_map(|(x, y)| [x, y]).max().unwrap() / 4;
    solution(&map, &path, &mut |p: Pos, f: Facing| -> (Pos, Facing) {
        let (x, y) = (p.0 - 1, p.1 - 1);
        let next_side = |xy: u8| (xy / side + 1) * side;
        let prev_side = |xy: u8| (xy / side - 1) * side;
        let wrap = |xy: u8| side - (xy % side) - 1;
        match f {
            Facing::Right => {
                let p2 = (
                    next_side(x) + wrap(y) + 1,
                    next_side(y) + wrap(x) + 1,
                );
                if map.contains_key(&p2) { return (p2, Facing::Down); }

                if y >= side {
                    let p2 = (
                        next_side(x) + y % side + 1,
                        prev_side(y) + side - 1 + 1,
                    );
                    if map.contains_key(&p2) { return (p2, Facing::Up); }
                }

                if y >= 2 * side {
                    let p2 = (
                        next_side(x) + side - 1 + 1,
                        prev_side(prev_side(y)) + wrap(y) + 1,
                    );
                    if map.contains_key(&p2) { return (p2, Facing::Left); }
                }

                if x >= side {
                    let p2 = (
                        prev_side(x) + side - 1 + 1,
                        next_side(next_side(y)) + wrap(y) + 1,
                    );
                    if map.contains_key(&p2) { return (p2, Facing::Left); }
                }

                unimplemented!("{},{} {}", p.0, p.1, facing_char(f))
            },
            Facing::Down => {
                if x >= 2 * side && y >= side {
                    let p2 = (
                        prev_side(x - side) + wrap(x) + 1,
                        prev_side(y) + side - 1 + 1,
                    );
                    if map.contains_key(&p2) { return (p2, Facing::Up); }
                }

                if x >= side {
                    let p2 = (
                        prev_side(x) + side - 1 + 1,
                        next_side(y) + x % side + 1,
                    );
                    if map.contains_key(&p2) { return (p2, Facing::Left); }
                }

                if y >= 3 * side {
                    let p2 = (
                        next_side(next_side(x)) + side % x + 1,
                        prev_side(prev_side(prev_side(y))) + 1,
                    );
                    if map.contains_key(&p2) { return (p2, Facing::Down); }
                }

                unimplemented!("{},{} {}", p.0, p.1, facing_char(f))
            },
            Facing::Left => {
                if x >= side {
                    let p2 = (
                        prev_side(x) + y % side + 1,
                        next_side(y) + 1,
                    );
                    if map.contains_key(&p2) { return (p2, Facing::Down); }
                }

                if x == 0 && y >= 3 * side {
                    let p2 = (
                        next_side(x) + y % side + 1,
                        prev_side(prev_side(prev_side(y))) + 1,
                    );
                    if map.contains_key(&p2) { return (p2, Facing::Down); }
                }

                if x == 0 && y >= 2 * side {
                    let p2 = (
                        next_side(x) + 1,
                        prev_side(prev_side(y)) + wrap(y) + 1,
                    );
                    if map.contains_key(&p2) { return (p2, Facing::Right); }
                }

                if x >= side {
                    let p2 = (
                        prev_side(x) + 1,
                        next_side(next_side(y)) + wrap(y) + 1,
                    );
                    if map.contains_key(&p2) { return (p2, Facing::Right); }
                }

                unimplemented!("{},{} {}", p.0, p.1, facing_char(f))
            },
            Facing::Up => {
                if y >= side {
                    let p2 = (
                        next_side(x) + y % side + 1,
                        prev_side(y) + x % side + 1,
                    );
                    if map.contains_key(&p2) { return (p2, Facing::Right); }
                }

                if x >= side && y == 0 {
                    let p2 = (
                        prev_side(x) + 1,
                        next_side(next_side(next_side(y))) + x % side + 1,
                    );
                    if map.contains_key(&p2) { return (p2, Facing::Right); }
                }

                if y == 0 {
                    let p2 = (
                        prev_side(prev_side(x)) + x % side + 1,
                        next_side(next_side(next_side(y))) + side - 1 + 1,
                    );
                    if map.contains_key(&p2) { return (p2, Facing::Up); }
                }

                unimplemented!("{},{} {}", p.0, p.1, facing_char(f))
            },
        }
    })
}

fn read_file(file_path: &str) -> String {
    let mut input_file = File::open(file_path).expect("file not found");
    let mut input = String::new();
    input_file.read_to_string(&mut input)
        .expect("something went wrong reading the file");
    input
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        let input = read_file("test.txt");
        assert_eq!(solution1(&input), 6032);
    }

    #[test]
    fn test_solution2() {
        let input = read_file("test.txt");
        assert_eq!(solution2(&input), 5031);
    }

    #[test]
    fn test_full_solution1() {
        let input = read_file("input");
        assert_eq!(solution1(&input), 57350);
    }

    #[test]
    fn test_full_solution2() {
        let input = read_file("input");
        assert_eq!(solution2(&input), 104385);
    }
}

fn main() {
    let input = read_file("input");

    let s1 = solution1(&input);
    println!("solution 1: {}", s1);

    let s2 = solution2(&input);
    println!("solution 2: {}", s2);
}
