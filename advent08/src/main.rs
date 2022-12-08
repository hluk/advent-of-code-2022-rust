use std::fs::File;
use std::io::prelude::*;

struct Map {
    map: Vec<u8>,
    width: usize,
}

impl Map {
    fn new(input: &str) -> Self {
        let map: Vec<u8> = input
            .trim_end()
            .chars()
            .filter_map(|c| c.to_digit(10))
            .map(|c| c as u8)
            .collect();
        let width = input.find('\n').unwrap_or(0);
        Map {map, width}
    }
}

struct View {
    from: u8,
    count: usize,
    max: u8,
}

impl View {
    fn new(from: u8) -> Self {
        View {from: from, count: 0, max: 0}
    }
}

fn view(v: View, y: &u8) -> View {
    if v.from <= v.max {
        v
    } else {
        View {
             from: v.from,
             count: v.count + 1,
             max: v.max.max(*y),
        }
    }
}

fn solution1(input: &str) -> usize {
    let map = Map::new(input);
    let l = map.map.len();
    let w = map.width;
    let h = l/w;
    map.map
        .iter()
        .enumerate()
        .filter(|(i, x)| {
            let row_start = i/w*h;
            let row_end = l.min((i+h)/w*h);
            let col = i%h;
            map.map[col..*i].iter().step_by(w).all(|y| y < x) ||
            map.map[row_start..*i].iter().all(|y| y < x) ||
            map.map[l.min(i+w)..l].iter().step_by(w).all(|y| y < x) ||
            map.map[i+1..row_end].iter().all(|y| y < x)
        })
        .count()
}

fn solution2(input: &str) -> usize {
    let map = Map::new(input);
    let l = map.map.len();
    let w = map.width;
    let h = l/w;
    map.map
        .iter()
        .enumerate()
        .map(|(i, x)| {
            let row_start = i/w*h;
            let row_end = l.min((i+h)/w*h);
            let col = i%h;
            map.map[col..i].iter().step_by(w).rev().fold(View::new(*x), view).count *
            map.map[row_start..i].iter().rev().fold(View::new(*x), view).count *
            map.map[l.min(i+w)..l].iter().step_by(w).fold(View::new(*x), view).count *
            map.map[i+1..row_end].iter().fold(View::new(*x), view).count
        })
        .max().unwrap_or(0)
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
        assert_eq!(solution1(&input), 21);
    }

    #[test]
    fn test_solution2() {
        let input = read_file("test.txt");
        assert_eq!(solution2(&input), 8);
    }
}

fn main() {
    let input = read_file("input");
    let s1 = solution1(&input);
    println!("solution 1: {}", s1);

    let s2 = solution2(&input);
    println!("solution 2: {}", s2);
}
