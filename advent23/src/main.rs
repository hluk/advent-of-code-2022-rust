use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;
use std::collections::HashSet;

type Pos = (i32, i32);
type Map = HashSet<Pos>;

fn parse(input: &str) -> Map {
    input.trim_end()
        .split('\n')
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .enumerate()
                .filter_map(move |(x, c)| {
                    if c == '#' {
                        Some((x as i32, y as i32))
                    } else {
                        None
                    }
                })
        })
        .collect()
}

fn solution(input: &str, rounds: usize) -> (usize, usize) {
    let mut m = parse(input);
    let mut dir = 0;
    let mut round = 0;
    loop {
        if round > 0 && round == rounds { break; }
        round += 1;

        let mut collide = HashMap::<Pos, Option<Pos>>::new();
        for &(x, y) in &m {
            let n = !m.contains(&(x, y - 1));
            let s = !m.contains(&(x, y + 1));
            let w = !m.contains(&(x - 1, y));
            let e = !m.contains(&(x + 1, y));
            let ne = !m.contains(&(x + 1, y - 1));
            let nw = !m.contains(&(x - 1, y - 1));
            let se = !m.contains(&(x + 1, y + 1));
            let sw = !m.contains(&(x - 1, y + 1));
            if n && s && w && e && ne && nw && se && sw { continue; }

            for i in 0..4 {
                let d = (dir + i) % 4;
                match d {
                    0 => if n && ne && nw {
                        collide.entry((x, y - 1)).and_modify(|e| *e = None).or_insert_with(|| Some((x, y)));
                        break;
                    },
                    1 => if s && se && sw {
                        collide.entry((x, y + 1)).and_modify(|e| *e = None).or_insert_with(|| Some((x, y)));
                        break;
                    },
                    2 => if w && nw && sw {
                        collide.entry((x - 1, y)).and_modify(|e| *e = None).or_insert_with(|| Some((x, y)));
                        break;
                    },
                    3 => if e && ne && se {
                        collide.entry((x + 1, y)).and_modify(|e| *e = None).or_insert_with(|| Some((x, y)));
                        break;
                    },
                    _ => unreachable!(),
                }
            }
        }

        dir = (dir + 1) % 4;

        let mut moved = false;
        for (to, from) in collide {
            if let Some(from) = from {
                m.remove(&from);
                m.insert(to);
                moved = true;
            }
        }
        if !moved { break; }
    }
    let x0 = m.iter().min().unwrap().0;
    let x1 = m.iter().max().unwrap().0;
    let y0 = m.iter().min_by_key(|p| p.1).unwrap().1;
    let y1 = m.iter().max_by_key(|p| p.1).unwrap().1;
    (((x1 - x0 + 1) * (y1 - y0 + 1)) as usize - m.len(), round)
}

fn solution1(input: &str) -> usize {
    solution(input, 10).0
}

fn solution2(input: &str) -> usize {
    solution(input, 0).1
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
        assert_eq!(solution1(&input), 110);
    }

    #[test]
    fn test_simple_solution1() {
        let input = read_file("test_simple.txt");
        assert_eq!(solution1(&input), 25);
    }

    #[test]
    fn test_solution2() {
        let input = read_file("test.txt");
        assert_eq!(solution2(&input), 20);
    }
}

fn main() {
    let input = read_file("input");

    let s1 = solution1(&input);
    println!("solution 1: {}", s1);

    let s2 = solution2(&input);
    println!("solution 2: {}", s2);
}
