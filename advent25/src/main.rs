use std::fs::File;
use std::io::prelude::*;

type Num = i64;
type Nums = Vec<Num>;

fn parse(input: &str) -> Nums {
    input.trim_end()
        .split('\n')
        .map(|line| {
            line.chars()
                .rev()
                .enumerate()
                .map(|(i, c)| {
                    (5 as Num).pow(i as u32) *
                    match c {
                        '0' => 0,
                        '1' => 1,
                        '2' => 2,
                        '-' => -1,
                        '=' => -2,
                        _ => panic!(),
                    }
                }).sum()
        })
        .collect()
}

fn to_snafu(i: Num) -> String
{
    let mut i = i;
    let l = (i.abs() as f64).log(5.0).floor() as usize + 1;
    let mut ns: Vec<i8> = (1..=l).rev().map(|l| {
        let m = (5 as Num).pow(l as u32 - 1);
        let n = i / m;
        i -= m * n;
        n as i8
    }).collect();

    let mut carry = 0;
    for n in ns.iter_mut().rev() {
        *n += carry;
        if *n > 2 {
            *n -= 5;
            carry = 1;
        } else {
            carry = 0;
        }
    }

    if carry != 0 || ns.is_empty() {
        ns.insert(0, carry);
    }

    let ds = b"=-012";
    ns.iter().map(|n| { ds[(n+2) as usize] as char }).collect()
}

fn solution1(input: &str) -> String {
    let ns = parse(input);
    let s = ns.iter().sum();
    to_snafu(s)
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
    fn test_to_snafu() {
        assert_eq!(to_snafu(0), "0");
        assert_eq!(to_snafu(1), "1");
        assert_eq!(to_snafu(2), "2");
        assert_eq!(to_snafu(3), "1=");
        assert_eq!(to_snafu(4), "1-");
        assert_eq!(to_snafu(2022), "1=11-2");
        assert_eq!(to_snafu(12345), "1-0---0");
        assert_eq!(to_snafu(314159265), "1121-1110-1=0");
        assert_eq!(to_snafu(4890), "2=-1=0");
    }

    #[test]
    fn test_solution1() {
        let input = read_file("test.txt");
        assert_eq!(solution1(&input), "2=-1=0");
    }
}

fn main() {
    let input = read_file("input");

    let s1 = solution1(&input);
    println!("solution 1: {}", s1);
}
