use std::fs::File;
use std::io::prelude::*;
use std::collections::HashMap;

const MAX_DIR_SIZE: u32 = 100000;
const TOTAL_SIZE: u32 = 70000000;
const NEEDED_SIZE: u32 = 30000000;

fn sizes(input: &str) -> Vec<u32> {
    let mut cwd = Vec::new();
    let mut sizes = HashMap::<Vec::<&str>, u32>::new();
    for line in input.lines() {
        if line == "$ ls" || line.starts_with("dir ") {
            continue
        }

        if line == "$ cd /" {
            cwd.clear();
        } else if line == "$ cd .." {
            cwd.pop();
        } else if line.starts_with("$ cd ") {
            cwd.push(line.strip_prefix("$ cd ").unwrap());
        } else {
            let x: Vec<&str> = line.split(' ').collect();
            let size = x[0].parse::<u32>().unwrap();
            for i in 0..=cwd.len() {
                let dir_size = sizes.entry(cwd[..i].to_vec()).or_insert(0);
                *dir_size += size;
            }
        }
    }
    sizes.values().cloned().collect()
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
