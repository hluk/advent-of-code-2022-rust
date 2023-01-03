use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

type Monkey = String;

#[derive(Copy, Clone, Debug, PartialEq)]
enum Operation { Add, Sub, Mul, Div }

#[derive(Clone, Debug, PartialEq)]
enum Job {
    Number(i64),
    Eval((Operation, Monkey, Monkey)),
}

type Monkeys = HashMap<Monkey, Job>;

fn parse(input: &str) -> Monkeys {
    input.trim_end().split('\n')
        .map(|line| {
            let mut it = line.split(": ");
            let monkey = it.next().unwrap();
            let rhs = it.next().unwrap();
            let mut it2 = rhs.split(' ');
            let a = it2.next().unwrap();
            if let Ok(number) = a.parse::<i64>() {
                (monkey.to_string(), Job::Number(number))
            } else {
                let op = it2.next().unwrap();
                let b = it2.next().unwrap();
                let operation = match op {
                    "+" => Operation::Add,
                    "-" => Operation::Sub,
                    "*" => Operation::Mul,
                    "/" => Operation::Div,
                    _ => panic!("Unexpected operation {}", op),
                };
                (monkey.to_string(), Job::Eval((operation, a.to_string(), b.to_string())))
            }
        }).collect()
}

fn eval(monkey: &Monkey, ms: &Monkeys) -> i64 {
    match &ms[monkey] {
        Job::Number(number) => *number,
        Job::Eval((Operation::Add, a, b)) => eval(a, ms) + eval(b, ms),
        Job::Eval((Operation::Sub, a, b)) => eval(a, ms) - eval(b, ms),
        Job::Eval((Operation::Mul, a, b)) => eval(a, ms) * eval(b, ms),
        Job::Eval((Operation::Div, a, b)) => eval(a, ms) / eval(b, ms),
    }
}

fn eval2(monkey: &Monkey, ms: &Monkeys) -> i64 {
    for (m, rhs) in ms {
        if m == "root" { continue; }
        return match rhs {
            Job::Eval((Operation::Add, a, b)) if a == monkey => eval2(m, ms) - eval(b, ms),
            Job::Eval((Operation::Sub, a, b)) if a == monkey => eval2(m, ms) + eval(b, ms),
            Job::Eval((Operation::Mul, a, b)) if a == monkey => eval2(m, ms) / eval(b, ms),
            Job::Eval((Operation::Div, a, b)) if a == monkey => eval2(m, ms) * eval(b, ms),

            Job::Eval((Operation::Add, a, b)) if b == monkey => eval2(m, ms) - eval(a, ms),
            Job::Eval((Operation::Sub, a, b)) if b == monkey => eval(a, ms) - eval2(m, ms),
            Job::Eval((Operation::Mul, a, b)) if b == monkey => eval2(m, ms) / eval(a, ms),
            Job::Eval((Operation::Div, a, b)) if b == monkey => eval(a, ms) / eval2(m, ms),
            _ => continue,
        }
    }
    match &ms["root"] {
        Job::Eval((_, a, b)) if a == monkey => eval(b, ms),
        Job::Eval((_, a, b)) if b == monkey => eval(a, ms),
        _ => panic!("Unexpected root"),
    }
}

fn solution1(input: &str) -> i64 {
    let ms = parse(input);
    eval(&"root".to_string(), &ms)
}

fn solution2(input: &str) -> i64 {
    let ms = parse(input);
    eval2(&"humn".to_string(), &ms)
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
        assert_eq!(solution1(&input), 152);
    }

    #[test]
    fn test_solution2() {
        let input = read_file("test.txt");
        assert_eq!(solution2(&input), 301);
    }
}

fn main() {
    let input = read_file("input");

    let s1 = solution1(&input);
    println!("solution 1: {}", s1);

    let s2 = solution2(&input);
    println!("solution 2: {}", s2);
}
