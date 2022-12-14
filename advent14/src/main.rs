use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

type Pos = (u16, u16);
type Map = HashSet<Pos>;
const SAND: Pos = (500, 0);

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

fn solution(map: &mut Map) -> usize {
    let block_count = map.len();
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
