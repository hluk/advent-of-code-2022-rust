use std::fs::File;
use std::io::prelude::*;

use std::collections::HashSet;

fn solution1(input: &String) -> u32 {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| -> u32 {
            let len = line.len();
            let l = len / 2;
            let p1: HashSet<u8> = line[0..l].as_bytes().iter().cloned().collect();
            let p2: HashSet<u8> = line[l..len].as_bytes().iter().cloned().collect();
            let both: HashSet<u8> = p1.intersection(&p2).cloned().collect();
            if both.len() != 1 {
                panic!("Exactly one item should be same in both compartments");
            }
            let b: u8 = both.iter().cloned().next().unwrap();
            if b >= 97 {
                b - 97 + 1
            } else {
                b - 65 + 27
            }.into()
        })
        .sum()
}

fn solution2(input: &String) -> u32 {
    let rucksacks: Vec<HashSet<u8>> = input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| -> HashSet<u8> {
            line.as_bytes().iter().cloned().collect()
        })
        .collect();
    rucksacks
        .chunks(3)
        .map(|x| -> u32 {
            let both: HashSet<u8> = x[0].intersection(&x[1]).cloned().collect();
            if both.len() == 0 {
                panic!("At least one item should be same in both compartments");
            }
            let all: HashSet<u8> = both.intersection(&x[2]).cloned().collect();
            if all.len() != 1 {
                panic!("Exactly one item should be same in all compartments");
            }
            let b: u8 = all.iter().cloned().next().unwrap();
            if b >= 97 {
                b - 97 + 1
            } else {
                b - 65 + 27
            }.into()
        })
        .sum()
}

fn read_file(file_path: &str) -> String {
    let mut input_file = File::open(file_path).expect("file not found");

    let mut input = String::new();
    input_file.read_to_string(&mut input)
        .expect("something went wrong reading the file");

    return input;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solution1() {
        let input = read_file("test.txt");
        assert_eq!(solution1(&input), 157);
    }

    #[test]
    fn test_solution2() {
        let input = read_file("test.txt");
        assert_eq!(solution2(&input), 70);
    }
}

fn main() {
    let input = read_file("input");
    let s1 = solution1(&input);
    println!("solution 1: {}", s1);

    let s2 = solution2(&input);
    println!("solution 2: {}", s2);
}
