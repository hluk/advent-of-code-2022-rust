use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;

type Pos = (i8, i8, i8);
const SIDES: [Pos; 6] = [
    (0,0,1),
    (0,1,0),
    (1,0,0),
    (0,0,-1),
    (0,-1,0),
    (-1,0,0),
];

fn parse(input: &str) -> HashSet<Pos> {
    input
        .trim_end()
        .split('\n')
        .map(|line| {
            // line: "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"
            let mut it = line.split(',').map(|x| x.parse::<i8>().unwrap());
            (
                it.next().unwrap(),
                it.next().unwrap(),
                it.next().unwrap(),
            )
        })
        .collect()
}

fn solution1(input: &str) -> usize {
    let ps = parse(input);
    ps.iter()
        .flat_map(|p| SIDES.map(|s| (p.0 + s.0, p.1 + s.1, p.2 + s.2)))
        .filter(|p| ps.iter().copied().find(|p2| *p == *p2).is_none())
        .count()
}

fn solution2(input: &str) -> usize {
    let ps = parse(input);

    let x_min = ps.iter().map(|p| p.0).min().unwrap() - 1;
    let x_max = ps.iter().map(|p| p.0).max().unwrap() + 1;
    let y_min = ps.iter().map(|p| p.1).min().unwrap() - 1;
    let y_max = ps.iter().map(|p| p.1).max().unwrap() + 1;
    let z_min = ps.iter().map(|p| p.2).min().unwrap() - 1;
    let z_max = ps.iter().map(|p| p.2).max().unwrap() + 1;

    let faces: HashSet<(Pos, Pos)> = ps.iter()
        .flat_map(|p| SIDES.map(|s| (*p, s)))
        .filter(|(p, s)| ps.iter().copied().find(|p2| (p.0 + s.0, p.1 + s.1, p.2 + s.2) == *p2).is_none())
        .collect();
    let mut external_faces = HashSet::<(Pos, Pos)>::new();

    let mut to_visit = Vec::<Pos>::new();
    let mut visited = HashSet::<Pos>::new();
    to_visit.push((0,0,0));
    while let Some(f) = to_visit.pop() {
        visited.insert(f);
        for s in SIDES {
            let p = (f.0 - s.0, f.1 - s.1, f.2 - s.2);

            if p.0 < x_min || p.0 > x_max
            || p.1 < y_min || p.1 > y_max
            || p.2 < z_min || p.2 > z_max
            {
                continue;
            }

            if !ps.contains(&p) {
                if !visited.contains(&p) {
                    to_visit.push(p);
                }
            } else if faces.contains(&(p, s)) {
                external_faces.insert((p, s));
            }
        }
    }

    external_faces.len()
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
        assert_eq!(solution1(&input), 64);
    }

    #[test]
    fn test_solution2() {
        let input = read_file("test.txt");
        assert_eq!(solution2(&input), 58);
    }

    #[test]
    fn test_solution2_input() {
        let input = read_file("input");
        let x = solution2(&input);
        assert!(x > 2536, "Value too low: {}", x);
    }
}

fn main() {
    let input = read_file("input");

    let s1 = solution1(&input);
    println!("solution 1: {}", s1);

    let s2 = solution2(&input);
    println!("solution 2: {}", s2);
}
