use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;
use std::{thread, time};

const START_X: usize = 2;
const START_Y: usize = 3;
const CHAMBER_WIDTH: usize = 7;
const RENDER_LINES: usize = 40;
const RENDER: bool = false;

type Pos = (usize, usize);
type Rock = Vec<Pos>;
type Rocks = Vec<Rock>;
type Chamber = HashSet<Pos>;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Move {
    Left,
    Right,
}
type Moves = Vec<Move>;

fn parse_rocks(input: &str) -> Rocks {
    input.trim_end()
        .split("\n\n")
        .map(|block| {
            block.split('\n')
                .enumerate()
                .flat_map(|(y, line)| {
                    line.chars()
                        .enumerate()
                        .filter(|(_x, c)| *c == '#')
                        .map(move |(x, _c)| (x, y))
                })
                .collect()
        })
        .collect()
}

fn parse_moves(input: &str) -> Moves {
    input.trim_end().chars()
        .map(|c| if c == '<' { Move::Left } else { Move::Right })
        .collect()
}

fn is_valid_position(pos: Pos, rock: &Rock, chamber: &Chamber) -> bool {
    !rock.iter().any(|p| {
        chamber.contains(&(p.0 + pos.0, pos.1 - p.1))
    })
}

fn render(rock_pos: Pos, rock: &Rock, chamber: &Chamber, top: usize) {
    if !RENDER { return; }

    let y2 = if top + START_Y < RENDER_LINES { RENDER_LINES } else { top + START_Y };
    let y1 = y2 - RENDER_LINES;

    let render: String = (y1..y2).rev().map(|y| {
        (0..7).map(|x| {
            let pos = (x, y);
            if rock.iter().any(|p| (p.0 + rock_pos.0, rock_pos.1 - p.1) == pos) {
                "\x1b[31;1;4m@\x1b[0m"
            } else if chamber.contains(&pos) {
                "#"
            } else if y == top {
                "\x1b[33;1;4m.\x1b[0m"
            } else {
                "."
            }
        }).collect::<String>()
    })
    .fold(String::new(), |a, b| a + &b + "\n");
    print!("{}", render);
    print!("\x1b[{}A", y2 - y1);
    thread::sleep(time::Duration::from_millis(50));
}

fn solution(input: &str, rocks: &Rocks, rock_count: usize) -> usize {
    let moves = parse_moves(input);
    let mut top = 0;
    let mut top_add = 0;
    let mut cache: HashMap<([u32; CHAMBER_WIDTH], usize), (usize, usize)> = HashMap::new();
    let mut chamber = HashSet::<Pos>::new();
    let mut move_index = 0;
    let mut r = 0;
    while r < rock_count {
        let rock = &rocks[r % rocks.len()];
        let width = rock.iter().map(|p| p.0).max().unwrap() + 1;
        let height = rock[rock.len() - 1].1 + 1;
        let mut pos = (START_X, top + height + START_Y - 1);
        loop {
            render(pos, &rock, &chamber, top);
            let m = moves[move_index];
            move_index = (move_index + 1) % moves.len();

            if m == Move::Left && pos.0 > 0 {
                let p2 = (pos.0 - 1, pos.1);
                if is_valid_position(p2, &rock, &chamber) { pos = p2; }
            } else if m == Move::Right && pos.0 + width < CHAMBER_WIDTH {
                let p2 = (pos.0 + 1, pos.1);
                if is_valid_position(p2, &rock, &chamber) { pos = p2; }
            }
            render(pos, &rock, &chamber, top);

            if pos.1 >= height {
                let p2 = (pos.0, pos.1 - 1);
                if is_valid_position(p2, &rock, &chamber) {
                    pos = p2;
                    continue;
                }
            }

            top = top.max(pos.1 + 1);
            for p in rock {
                chamber.insert((pos.0 + p.0, pos.1 - p.1));
            }

            if top >= 32 {
                let tops: [u32; CHAMBER_WIDTH] = core::array::from_fn(|x| {
                    (0..32).map(|y| {
                        if chamber.contains(&(x, top - y)) {
                            1 << y
                        } else {
                            0
                        }
                    }).sum()
                });
                let cache_key = (tops, r % rock.len());
                if let Some((r2, top2)) = cache.get(&cache_key) {
                    let d = r - r2;
                    let times = (rock_count - r2) / d;
                    top_add += (top - top2) * (times - 1);
                    r = r2 + d * times;
                } else {
                    cache.insert(cache_key, (r, top));
                }
            }
            break;
        }

        r += 1;
    }
    top + top_add
}

fn solution1(input: &str, rocks: &Rocks) -> usize {
    solution(input, rocks, 2022)
}

fn solution2(input: &str, rocks: &Rocks) -> usize {
    solution(input, rocks, 1_000_000_000_000)
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
        let rocks = parse_rocks(&read_file("rocks.txt"));
        let input = read_file("test.txt");
        assert_eq!(solution1(&input, &rocks), 3068);
    }

    #[test]
    fn test_solution2() {
        let rocks = parse_rocks(&read_file("rocks.txt"));
        let input = read_file("test.txt");
        assert_eq!(solution2(&input, &rocks), 1514285714288);
    }
}

fn main() {
    let rocks = parse_rocks(&read_file("rocks.txt"));
    let input = read_file("input");

    let s1 = solution1(&input, &rocks);
    println!("solution 1: {}", s1);

    let s2 = solution2(&input, &rocks);
    println!("solution 2: {}", s2);
}
