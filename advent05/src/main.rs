use std::fs::File;
use std::io::prelude::*;

struct Move {
    amount: u8,
    from: u8,
    to: u8,
}

type Crates = Vec<char>;
type Stacks = [Crates; 9];

struct Plan {
    stacks: Stacks,
    moves: Vec<Move>,
}

fn parse_stacks(input: &str) -> Stacks {
    let crate_lines: Vec<String> = input
        .split('\n')
        .map(|line| {
            line.chars()
                .skip(1)
                .step_by(4)
                .collect()
        })
        .collect();

    let mut stacks: Stacks = core::array::from_fn(|_i| Vec::new());
    for crate_line in crate_lines[0..crate_lines.len() - 1].iter() {
        for (stack, c) in crate_line.chars().enumerate().filter(|(_i, c)| *c != ' ') {
            stacks[stack].insert(0, c)
        }
    }
    stacks
}

fn parse_moves(input: &str) -> Vec<Move> {
    input
        .split('\n')
        .map(|line| {
            line.split(' ')
                .skip(1)
                .step_by(2)
                .map(|x| x.parse::<u8>().unwrap())
                .collect()
        })
        .map(|xs: Vec<u8>| {
            Move {
                amount: xs[0],
                from: xs[1] - 1,
                to: xs[2] - 1,
            }
        })
        .collect()
}

impl Plan {
    fn from_string(input: &str) -> Plan {
        let parts: Vec<&str> = input
            .trim_end()
            .split("\n\n")
            .collect();

        Plan {
            stacks: parse_stacks(parts[0]),
            moves: parse_moves(parts[1]),
        }
    }
}

fn top(stacks: &Stacks) -> String {
    stacks.iter()
        .filter(|x| !x.is_empty())
        .map(|x| x[x.len() - 1])
        .collect()
}

fn move_crates(plan: &Plan, grab_multiple: bool) -> String {
    let mut stacks = plan.stacks.clone();
    for m in &plan.moves {
        let from = &mut stacks[m.from as usize];
        let index = from.len() - m.amount as usize;
        let to_move_iter = from.drain(index..);
        let to_move: Crates = if grab_multiple {
            to_move_iter.collect()
        } else {
            to_move_iter.rev().collect()
        };

        let to = &mut stacks[m.to as usize];
        to.extend(to_move);
    }
    top(&stacks)
}

fn solution1(plan: &Plan) -> String {
    move_crates(plan, false)
}

fn solution2(plan: &Plan) -> String {
    move_crates(plan, true)
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
        let plan = Plan::from_string(&input);
        assert_eq!(solution1(&plan), "CMZ");
    }

    #[test]
    fn test_solution2() {
        let input = read_file("test.txt");
        let plan = Plan::from_string(&input);
        assert_eq!(solution2(&plan), "MCD");
    }
}

fn main() {
    let input = read_file("input");
    let plan = Plan::from_string(&input);
    let s1 = solution1(&plan);
    println!("solution 1: {}", s1);

    let s2 = solution2(&plan);
    println!("solution 2: {}", s2);
}
