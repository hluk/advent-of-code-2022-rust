use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::Hash;

type Cost = [usize; 3];

type BluePrint = [Cost; 4];

type BluePrints = Vec<BluePrint>;

fn parse_blueprints(input: &str) -> BluePrints {
    input.trim_end()
        .split('\n')
        .map(|line| {
            let mut it = line.split("costs ").skip(1);
            let mut ore_robot = it.next().unwrap().split(' ');
            let ore_robot_cost = [
                ore_robot.next().unwrap().parse::<usize>().unwrap(),
                0,
                0,
            ];
            let mut clay_robot = it.next().unwrap().split(' ');
            let clay_robot_cost = [
                clay_robot.next().unwrap().parse::<usize>().unwrap(),
                0,
                0,
            ];
            let mut obsidian_robot = it.next().unwrap().split(' ');
            let obsidian_robot_cost = [
                obsidian_robot.next().unwrap().parse::<usize>().unwrap(),
                obsidian_robot.skip(2).next().unwrap().parse::<usize>().unwrap(),
                0,
            ];
            let mut geode_robot = it.next().unwrap().split(' ');
            let geode_robot_cost = [
                geode_robot.next().unwrap().parse::<usize>().unwrap(),
                0,
                geode_robot.skip(2).next().unwrap().parse::<usize>().unwrap(),
            ];
            [
                ore_robot_cost,
                clay_robot_cost,
                obsidian_robot_cost,
                geode_robot_cost,
            ]
        })
        .collect()
}

fn can_build(c: &Cost, other: &Cost) -> bool {
    c[0] >= other[0] && c[1] >= other[1] && c[2] >= other[2]
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct State {
    robots: [usize; 4],
    c: Cost,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.robots[3].cmp(&other.robots[3])
            .then_with(|| self.robots[2].cmp(&other.robots[2]))
            .then_with(|| self.robots[1].cmp(&other.robots[1]))
            .then_with(|| self.robots[0].cmp(&other.robots[0]))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct States {
    visited: HashMap<(State, usize), usize>,
    heap: BinaryHeap<(usize, usize, State)>,
}

impl States {
    fn new() -> Self
    {
        let visited = HashMap::new();
        let heap = BinaryHeap::new();
        States {visited, heap}
    }

    fn visit(&mut self, state: &State, minutes: usize, geodes: usize)
    {
        if let Some(geodes2) = self.visited.get(&(*state, minutes)).copied() {
            if geodes2 >= geodes { return; }
        }

        self.visited.insert((*state, minutes), geodes);
        self.heap.push((geodes, minutes, *state));
    }
}

fn maximize_geodes(blueprint: &BluePrint, minutes: usize) -> usize {
    let mut visited = States::new();
    let start_state = State {robots: [1, 0, 0, 0], c: [0; 3]};
    visited.visit(&start_state, minutes, 0);
    let mut max_geodes = 0;

    let mut i = 0;
    'outer: while let Some((geodes, minutes, s)) = visited.heap.pop() {
        if i > 2000000 { break; }
        i += 1;

        if minutes == 0 {
            max_geodes = max_geodes.max(geodes);
            continue;
        }

        let c = [
            s.c[0] + s.robots[0],
            s.c[1] + s.robots[1],
            s.c[2] + s.robots[2],
        ];
        let geodes = geodes + s.robots[3];

        for i in (0..4).rev() {
            // Avoid building too many non-geode robots.
            if i < 3 && s.robots[i] >= blueprint[i + 1][i] { continue; }
            if can_build(&s.c, &blueprint[i]) {
                let mut robots = s.robots;
                robots[i] += 1;
                let c = [c[0] - blueprint[i][0], c[1] - blueprint[i][1], c[2] - blueprint[i][2]];
                let s2 = State {robots, c};
                visited.visit(&s2, minutes - 1, geodes);
                // Always prefer building geode robot.
                if i == 3 { continue 'outer; }
            }
        }

        let s2 = State {robots: s.robots, c: c};
        visited.visit(&s2, minutes - 1, geodes);
    }
    max_geodes
}

fn solution1(input: &str) -> usize {
    let bs = parse_blueprints(input);
    bs.iter().enumerate().map(|(i, b)| (i+1) * maximize_geodes(b, 24) ).sum()
}

fn solution2(input: &str) -> usize {
    let bs = parse_blueprints(input);
    bs[0..3.min(bs.len())].iter().map(|b| maximize_geodes(b, 32) ).product()
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
        assert_eq!(solution1(&input), 33);
    }

    #[test]
    fn test_solution2() {
        let input = read_file("test.txt");
        assert_eq!(solution2(&input), 56 * 62);
    }

    #[test]
    fn test_solution1_full() {
        let input = read_file("input");
        assert_eq!(solution1(&input), 1675);
    }

    #[test]
    fn test_solution2_full() {
        let input = read_file("input");
        assert_eq!(solution2(&input), 6840);
    }
}

fn main() {
    let input = read_file("input");

    let s1 = solution1(&input);
    println!("solution 1: {}", s1);

    let s2 = solution2(&input);
    println!("solution 2: {}", s2);
}
