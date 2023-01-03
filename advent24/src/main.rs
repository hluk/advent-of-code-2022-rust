use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::collections::HashSet;
use std::mem;

const RENDER: bool = false;

const UP: u8 = 0b1;
const DOWN: u8 = 0b10;
const LEFT: u8 = 0b100;
const RIGHT: u8 = 0b1000;

type Pos = (usize, usize);
type Entities = u8;
struct Map {
    m: HashMap<Pos, Entities>,
    w: usize,
    h: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct State {
    minutes: usize,
    pos: Pos,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.minutes.cmp(&self.minutes)
            .then_with(|| other.pos.cmp(&self.pos))
    }
}

fn parse(input: &str) -> Map {
    let m: HashMap<Pos, Entities> = input.trim_end()
        .split('\n')
        .skip(1)
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars()
                .skip(1)
                .enumerate()
                .filter_map(move |(x, c)| {
                    let p = (x, y);
                    match c {
                        '^' => Some((p, UP)),
                        'v' => Some((p, DOWN)),
                        '<' => Some((p, LEFT)),
                        '>' => Some((p, RIGHT)),
                        _ => None,
                    }
                })
        })
        .collect();

    let w = m.keys().max().unwrap().0 + 1;
    let h = m.keys().max_by_key(|p| p.1).unwrap().1 + 1;

    Map{m, w, h}
}

fn render(m: &Map, p: Pos) {
    if !RENDER { return; }

    let render: String = (0..m.h).map(|y| {
        (0..m.w).map(|x| {
            let p2 = (x, y);
            if p2 == p {
                "X"
            } else {
                let v = m.m.get(&p2);
                match v {
                    Some(&UP) => "^",
                    Some(&DOWN) => "v",
                    Some(&LEFT) => "<",
                    Some(&RIGHT) => ">",
                    Some(_) => "o",
                    None => ".",
                }
            }
        }).collect::<String>()
    })
    .fold(String::new(), |a, b| a + &b + "\n");
    println!("{}", render);
}

fn simulate(m: &mut Map) {
    let mut m2 = HashMap::new();
    mem::swap(&mut m.m, &mut m2);
    for ((x, y), v) in m2 {
        if v & UP != 0 {
            if y > 0 {
                m.m.entry((x, y - 1)).and_modify(|e| *e |= UP).or_insert(UP);
            } else {
                m.m.entry((x, m.h - 1)).and_modify(|e| *e |= UP).or_insert(UP);
            }
        }

        if v & DOWN != 0 {
            m.m.entry((x, (y + 1) % m.h)).and_modify(|e| *e |= DOWN).or_insert(DOWN);
        }

        if v & LEFT != 0 {
            if x > 0 {
                m.m.entry((x - 1, y)).and_modify(|e| *e |= LEFT).or_insert(LEFT);
            } else {
                m.m.entry((m.w - 1, y)).and_modify(|e| *e |= LEFT).or_insert(LEFT);
            }
        }

        if v & RIGHT != 0 {
            m.m.entry(((x + 1) % m.w, y)).and_modify(|e| *e |= RIGHT).or_insert(RIGHT);
        }
    }
}

fn gcd(a: usize, b: usize) -> usize {
    if b != 0 {
        gcd(b, a % b)
    } else {
        a
    }
}

fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        0
    } else {
        (a * b) / gcd(a, b)
    }
}

fn solution(m: &mut Map, start: Pos, exit: Pos) -> usize {
    let mut minutes = 0;
    let mut visited = HashSet::<(Pos, usize)>::new();
    let repeat = lcm(m.w, m.h);

    // wait until we can move
    while m.m.contains_key(&start) {
        render(m, (m.w, m.h));
        simulate(m);
        minutes += 1;
    }

    let mut heap = BinaryHeap::<State>::new();
    heap.push(State{pos: start, minutes});
    while let Some(s) = heap.pop() {
        if RENDER {
            println!("{} {} {},{}", minutes, s.minutes, s.pos.0, s.pos.1);
        }

        render(m, s.pos);
        if minutes != s.minutes {
            simulate(m);
            minutes += 1;
        }
        if minutes != s.minutes { panic!("Unexpected"); }
        if s.pos == exit { return dbg!(s.minutes); }

        let (x, y) = s.pos;
        let v = minutes % repeat;
        if visited.contains(&(s.pos, v)) { continue; }
        visited.insert((s.pos, v));

        if !m.m.contains_key(&s.pos) {
            heap.push(State{pos: s.pos, minutes: minutes + 1});
        }
        if y + 1 < m.h && !m.m.contains_key(&(x, y + 1)) {
            heap.push(State{pos: (x, y + 1), minutes: minutes + 1});
        }
        if y > 0 && !m.m.contains_key(&(x, y - 1)) {
            heap.push(State{pos: (x, y - 1), minutes: minutes + 1});
        }
        if x + 1 < m.w && !m.m.contains_key(&(x + 1, y)) {
            heap.push(State{pos: (x + 1, y), minutes: minutes + 1});
        }
        if x > 0 && !m.m.contains_key(&(x - 1, y)) {
            heap.push(State{pos: (x - 1, y), minutes: minutes + 1});
        }
    }

    panic!();
}

fn solution1(input: &str) -> usize {
    let mut m = parse(input);
    let start = (0, 0);
    let exit = (m.w - 1, m.h - 1);
    solution(&mut m, start, exit)
}

fn solution2(input: &str) -> usize {
    let mut m = parse(input);
    let start = (0, 0);
    let exit = (m.w - 1, m.h - 1);
    solution(&mut m, start, exit)
    + solution(&mut m, exit, start)
    + solution(&mut m, start, exit)
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
        assert_eq!(solution1(&input), 18);
    }

    #[test]
    fn test_solution2() {
        let input = read_file("test.txt");
        assert_eq!(solution2(&input), 54);
    }

    #[test]
    fn test_full_solution1() {
        let input = read_file("input");
        assert_eq!(solution1(&input), 271);
    }

    #[test]
    fn test_full_solution2() {
        let input = read_file("input");
        assert_eq!(solution2(&input), 813);
    }
}

fn main() {
    let input = read_file("input");

    let s1 = solution1(&input);
    println!("solution 1: {}", s1);

    let s2 = solution2(&input);
    println!("solution 2: {}", s2);
}
