use std::fs::File;
use std::io::prelude::*;

fn solution1(input: &String) -> u32 {
    input
        .split("\n\n")
        .map(|lines| {
            lines
                .split('\n')
                .filter(|line| !line.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .sum()
        })
        .max().unwrap()
}

fn solution2(input: &String) -> u32 {
    let mut elves : Vec<u32> = input
        .split("\n\n")
        .map(|lines| {
            lines
                .split('\n')
                .filter(|line| !line.is_empty())
                .map(|x| x.parse::<u32>().unwrap())
                .sum()
        }).collect();
    elves.sort_unstable();
    elves.reverse();
    elves[0..3].iter().sum()
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
        assert_eq!(solution1(&input), 24000);
    }

    #[test]
    fn test_solution2() {
        let input = read_file("test.txt");
        assert_eq!(solution2(&input), 45000);
    }
}

fn main() {
    let input = read_file("input");
    let s1 = solution1(&input);
    println!("solution 1: {}", s1);

    let s2 = solution2(&input);
    println!("solution 2: {}", s2);
}
