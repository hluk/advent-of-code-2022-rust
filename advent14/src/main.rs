use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

type Pos = (i32, i32);
type Map = HashSet<Pos>;
const SAND: Pos = (500, 0);

fn parse_map(input: &str) -> Map {
    input
        .trim_end()
        .split('\n')
        .flat_map(|line| {
            let ps: Vec<Pos> = line.split(" -> ")
                .map(|pos| {
                    let mut it = pos.split(',').map(|x| x.parse::<i32>().unwrap());
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
        let mut land = (SAND.0, land_y);

        loop {
            while !map.contains(&(land.0, land.1)) {
                land = (land.0, land.1 + 1);
                if land.1 > max_y {
                    return map.len() - block_count;
                }
            }

            if !map.contains(&(land.0 - 1, land.1)) {
                land = (land.0 - 1, land.1 + 1);
            } else if !map.contains(&(land.0 + 1, land.1)) {
                land = (land.0 + 1, land.1 + 1);
            } else {
                break;
            }
        }

        land = (land.0, land.1 - 1);
        land_y = land_y.min(land.1);
        map.insert(land);

        if land == SAND {
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
