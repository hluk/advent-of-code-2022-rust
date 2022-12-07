use std::fs::File;
use std::io::prelude::*;

const MAX_DIR_SIZE: u32 = 100000;
const TOTAL_SIZE: u32 = 70000000;
const NEEDED_SIZE: u32 = 30000000;

fn sizes(input: &str) -> Vec<u32> {
    let mut sizes = Vec::new();
    let mut path_length = 0;
    for line in input.lines() {
        if line.chars().next().unwrap().is_ascii_digit() {
            let size = line.split(' ').next().unwrap().parse::<u32>().unwrap();
            sizes[path_length - 1] += size;
        } else if line == "$ cd .." {
            path_length -= 1;
            sizes[path_length - 1] += sizes[path_length];
        } else if line.starts_with("$ cd ") {
            sizes.insert(path_length, 0);
            path_length += 1;
        }
    }
    for i in 1..path_length {
        sizes[path_length - i - 1] += sizes[path_length - i];
    }
    sizes
}

fn solution1(input: &str) -> u32 {
    sizes(input).iter()
        .filter(|&&size| size <= MAX_DIR_SIZE)
        .sum()
}

fn solution2(input: &str) -> u32 {
    let sizes = sizes(input);
    let free = TOTAL_SIZE - sizes.iter().max().unwrap();
    let to_free = NEEDED_SIZE - free;

    *sizes.iter()
        .filter(|&&size| size >= to_free)
        .min_by_key(|&&size| size - to_free)
        .unwrap()
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
        assert_eq!(solution1(&input), 95437);
    }

    #[test]
    fn test_solution2() {
        let input = read_file("test.txt");
        assert_eq!(solution2(&input), 24933642);
    }
}

fn main() {
    let input = read_file("input");
    let s1 = solution1(&input);
    println!("solution 1: {}", s1);

    let s2 = solution2(&input);
    println!("solution 2: {}", s2);
}
