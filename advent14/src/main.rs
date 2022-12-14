use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::{thread, time};

type Pos = (u16, u16);
type Map = HashSet<Pos>;
const SAND: Pos = (500, 0);
const RENDER_COLUMNS: u16 = 200;
const RENDER_LINES: u16 = 50;
const OFFSET: u16 = RENDER_LINES / 10;

fn parse_map(input: &str) -> Map {
    input
        .trim_end()
        .split('\n')
        .flat_map(|line| {
            let ps: Vec<Pos> = line.split(" -> ")
                .map(|pos| {
                    let mut it = pos.split(',').map(|x| x.parse::<u16>().unwrap());
                    (it.next().unwrap(), it.next().unwrap())
                }).collect();
            ps.windows(2).flat_map(|p2| -> Vec<Pos> {
                let (x1, y1) = p2[0];
                let (x2, y2) = p2[1];
                if x1 < x2 {
                    (x1..=x2).map(|x| (x, y2)).collect()
                } else if x1 > x2 {
                    (x2..=x1).map(|x| (x, y2)).collect()
                } else if y1 < y2 {
                    (y1..=y2).map(|y| (x2, y)).collect()
                } else {
                    (y2..=y1).map(|y| (x2, y)).collect()
                }
            }).collect::<Vec<Pos>>()
        }).collect()
}

fn render_static(map: &Map, blocks: &Map, current: Pos, render_pos: &Pos) {
    let x1 = if render_pos.0 < RENDER_COLUMNS/2 { 0 } else { render_pos.0 - RENDER_COLUMNS/2 };
    let x2 = x1 + RENDER_COLUMNS;

    let y1 = if render_pos.1 < RENDER_LINES/2 { 0 } else { render_pos.1 - RENDER_LINES/2 };
    let y2 = y1 + RENDER_LINES;

    let render: String = (y1..y2).map(|y| {
        (x1..x2).map(|x| {
            let pos = (x, y);
            let c = if blocks.contains(&pos) {
                '#'
            } else if map.contains(&pos) {
                'o'
            } else if pos == SAND {
                '+'
            } else {
                '.'
            };
            if pos == current {
                format!("\x1b[31;1;4m{}\x1b[0m", c)
            } else if c == 'o' {
                format!("\x1b[33;1;4m{}\x1b[0m", c)
            } else if c == '+' {
                format!("\x1b[34;1;4m{}\x1b[0m", c)
            } else {
                format!("{}", c)
            }
        }).collect::<String>()
    })
    .fold(String::new(), |a, b| a + &b + "\n");
    print!("{}", render);
    print!("\x1b[{}A", y2 - y1 + 1);
    thread::sleep(time::Duration::from_millis(1));
}

fn render(map: &Map, blocks: &Map, current: Pos, render_pos: &mut Pos) {
    let mut scroll = true;
    while scroll {
        render_static(map, blocks, current, render_pos);

        scroll = false;
        if current.0 + RENDER_COLUMNS/2 <= render_pos.0 + OFFSET || current.0 + OFFSET >= render_pos.0 + RENDER_COLUMNS/2 {
            scroll = true;
            if render_pos.0 < current.0 {
                *render_pos = (render_pos.0 + OFFSET, render_pos.1);
            } else {
                *render_pos = (render_pos.0 - OFFSET, render_pos.1);
            }
        }

        if current.1 + RENDER_LINES/2 <= render_pos.1 + OFFSET || current.1 + OFFSET >= render_pos.1 + RENDER_LINES/2 {
            scroll = true;
            if render_pos.1 < current.1 {
                *render_pos = (render_pos.0, render_pos.1 + OFFSET);
            } else {
                *render_pos = (render_pos.0, render_pos.1 - OFFSET);
            }
        }
    }
}

fn solution(map: &mut Map) -> usize {
    let block_count = map.len();
    let blocks = map.clone();
    let mut render_pos = (SAND.0, SAND.1 + RENDER_LINES/2);
    let max_y = map.iter().max_by_key(|p| p.1).unwrap().1;
    let mut land_y = map.iter()
        .filter(|p| p.0 == SAND.0)
        .min_by_key(|p| {
            p.1 - SAND.1
        }).unwrap().1;

    loop {
        let (mut x, mut y) = (SAND.0, land_y);

        loop {
            if !map.contains(&(x, y)) {
                y += 1;
                if y > max_y {
                    return map.len() - block_count;
                }
            } else if !map.contains(&(x - 1, y)) {
                x -= 1;
            } else if !map.contains(&(x + 1, y)) {
                x += 1;
            } else {
                break;
            }
        }

        y -= 1;
        land_y = land_y.min(y);
        map.insert((x, y));

        if (x, y) == SAND {
            return map.len() - block_count;
        }

        render(&map, &blocks, (x, y), &mut render_pos);
    }
}

fn solution1(input: &str) -> usize {
    let mut map = parse_map(input);
    solution(&mut map)
}

fn solution2(input: &str) -> usize {
    let mut map = parse_map(input);

    // Add floor
    let max_y = map.iter().max_by_key(|p| p.1).unwrap().1 + 2;
    for x in 0..=1000 {
        map.insert((x, max_y));
    }

    solution(&mut map)
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
        assert_eq!(solution1(&input), 24);
    }

    #[test]
    fn test_solution2() {
        let input = read_file("test.txt");
        assert_eq!(solution2(&input), 93);
    }
}

fn main() {
    let input = read_file("input");
    let s1 = solution1(&input);
    println!("solution 1: {}", s1);

    let s2 = solution2(&input);
    println!("solution 2: {}", s2);
}
