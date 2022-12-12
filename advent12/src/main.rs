use core::cmp::Reverse;
use std::fs::File;
use std::io::prelude::*;
use std::collections::BinaryHeap;
use std::collections::HashSet;

type Pos = (u8, u8);

struct Map {
    m: Vec<Vec<char>>
}

impl Map {
    fn from(input: &str) -> Self {
        Map { m: input
            .trim_end()
            .split('\n')
            .map(|line| line.chars().collect())
            .collect()
        }
    }

    fn at(&self, p: Pos) -> char {
        self.m[p.1 as usize][p.0 as usize]
    }

    fn start(&self) -> Option<Pos> {
        self.m.iter().enumerate().find_map(|(y, row)| {
            row.iter().position(|&x| x == 'S').map(|x| (x as u8, y as u8))
        })
    }

    fn neighbors(&self, p: Pos) -> Vec<Pos> {
        let mut ps = Vec::<Pos>::with_capacity(4);
        if p.0 + 1 < self.m[p.1 as usize].len() as u8 { ps.push((p.0 + 1, p.1)); }
        if p.1 + 1 < self.m.len() as u8 { ps.push((p.0, p.1 + 1)); }
        if p.0 > 0 { ps.push((p.0 - 1, p.1)); }
        if p.1 > 0 { ps.push((p.0, p.1 - 1)); }
        ps
    }

    fn steps(&self, start: Pos, max: usize) -> usize {
        let mut ps = HashSet::<Pos>::new();
        let mut heap = BinaryHeap::<(Reverse<usize>, Pos)>::new();
        ps.insert(start);
        heap.push((Reverse(0), start));

        while let Some((Reverse(d), p)) = heap.pop() {
            let h = self.at(p);
            if d == max || h == 'E' { return d; }

            let d2 = d + 1;
            for p2 in self.neighbors(p) {
                let h2 = self.at(p2);
                let a = height(h);
                let b = height(h2);
                if (b <= a || b - a <= 1) && !ps.contains(&p2) {
                    heap.push((Reverse(d2), p2));
                    ps.insert(p2);
                }
            }
        }

        max
    }
}

fn height(c: char) -> u8 {
    match c {
        'S' => b'a',
        'E' => b'z',
        _ => c as u8,
    }
}

fn solution1(input: &str) -> usize {
    let map = Map::from(input);
    let start = map.start().unwrap();
    map.steps(start, usize::MAX)
}

fn solution2(input: &str) -> usize {
    let map = Map::from(input);
    let mut min = usize::MAX;
    for y in 0..map.m.len() {
        for x in 0..map.m[y].len() {
            let pos = (x as u8, y as u8);
            // Visit only 'a' positions which have at least one non-'a' neighbor.
            if height(map.at(pos)) == b'a' && !map.neighbors(pos).iter().all(|&p| height(map.at(p)) == b'a') {
                min = map.steps(pos, min);
            }
        }
    }
    min
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
    fn test_map_start1() {
        let map = Map::from("\
            aaa\n\
            aaa\n\
            aSa\n\
            aaa\n\
        ");
        assert_eq!(map.start(), Some((1, 2)));
    }

    #[test]
    fn test_map_start2() {
        let input = read_file("test.txt");
        let map = Map::from(&input);
        assert_eq!(map.start(), Some((0, 0)));
    }

    #[test]
    fn test_solution1() {
        let input = read_file("test.txt");
        assert_eq!(solution1(&input), 31);
    }

    #[test]
    fn test_solution2() {
        let input = read_file("test.txt");
        assert_eq!(solution2(&input), 29);
    }
}

fn main() {
    let input = read_file("input");
    let s1 = solution1(&input);
    println!("solution 1: {}", s1);

    let s2 = solution2(&input);
    println!("solution 2: {}", s2);
}
