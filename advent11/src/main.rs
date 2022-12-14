use std::fs::File;
use std::io::prelude::*;
use std::mem;

type Operation = Box<dyn FnMut(u64) -> u64>;

struct Monkey {
    items: Vec<u64>,
    operation: Operation,
    test_div: u64,
    throw: [usize; 2],
    inspect_count: usize,
}

impl Monkey {
    fn from_input(input: &str) -> Self {
        let mut line = input.split('\n').skip(1);

        let items = line.next().unwrap()
            .strip_prefix("  Starting items: ").unwrap()
            .split(", ")
            .map(|x| x.parse::<u64>().unwrap())
            .collect();

        let op = line.next().unwrap()
            .strip_prefix("  Operation: new = old ").unwrap();
        let operand = op[2..op.len()].parse::<u64>().unwrap_or(0);
        let operation: Operation = if op == "* old" {
            Box::new(|old| old * old)
        } else if op.starts_with('+') {
            Box::new(move |old| old + operand)
        } else if op.starts_with('*') {
            Box::new(move |old| old * operand)
        } else {
            panic!("Unexpected operation: {}", op);
        };

        let test_div = line.next().unwrap()
            .strip_prefix("  Test: divisible by ").unwrap()
            .parse::<u64>().unwrap();
        let throw_true = line.next().unwrap()
            .strip_prefix("    If true: throw to monkey ").unwrap()
            .parse::<usize>().unwrap();
        let throw_false = line.next().unwrap()
            .strip_prefix("    If false: throw to monkey ").unwrap()
            .parse::<usize>().unwrap();

        Monkey {
            items,
            operation,
            test_div,
            throw: [throw_false, throw_true],
            inspect_count: 0,
        }
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    input
        .trim_end()
        .split("\n\n")
        .map(Monkey::from_input)
        .collect()
}

fn solution<Transform: Fn(u64) -> u64>(monkeys: &mut Vec<Monkey>, rounds: usize, transform: Transform) -> usize {
    for _round in 0..rounds {
        for i in 0..monkeys.len() {
            let mut items = Vec::<u64>::new();
            mem::swap(&mut monkeys[i].items, &mut items);
            monkeys[i].inspect_count += items.len();
            for old in items {
                let item = transform((monkeys[i].operation)(old));
                let target_monkey = monkeys[i].throw[(item % monkeys[i].test_div == 0) as usize];
                monkeys[target_monkey].items.push(item);
            }
        }
    }

    monkeys.iter().fold((1, 1, 1), |x, m| {
        let c = m.inspect_count.max(x.2);
        (x.0 / x.2 * c, c.max(x.1), c.min(x.1))
    }).0
}

fn solution1(input: &str) -> usize {
    let mut monkeys = parse_monkeys(input);
    solution(&mut monkeys, 20, |x| x / 3)
}

fn solution2(input: &str) -> usize {
    let mut monkeys = parse_monkeys(input);
    let d: u64 = monkeys.iter().map(|m| m.test_div).product();
    solution(&mut monkeys, 10000, |x| x % d)
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
        assert_eq!(solution1(&input), 10605);
    }

    #[test]
    fn test_solution2() {
        let input = read_file("test.txt");
        assert_eq!(solution2(&input), 2713310158);
    }
}

fn main() {
    let input = read_file("input");
    let s1 = solution1(&input);
    println!("solution 1: {}", s1);

    let s2 = solution2(&input);
    println!("solution 2: {}", s2);
}
