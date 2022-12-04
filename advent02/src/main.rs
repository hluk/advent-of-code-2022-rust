use std::fs::File;
use std::io::prelude::*;

fn solution1(input: &String) -> u32 {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            match line {
                "A X" => 3 + 1,
                "A Y" => 6 + 2,
                "A Z" => 0 + 3,
                "B X" => 0 + 1,
                "B Y" => 3 + 2,
                "B Z" => 6 + 3,
                "C X" => 6 + 1,
                "C Y" => 0 + 2,
                "C Z" => 3 + 3,
                _ => panic!("Unexpected")
            }
        })
        .sum()
}

fn solution2(input: &String) -> u32 {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            match line {
                "A X" => 0 + 3,
                "A Y" => 3 + 1,
                "A Z" => 6 + 2,
                "B X" => 0 + 1,
                "B Y" => 3 + 2,
                "B Z" => 6 + 3,
                "C X" => 0 + 2,
                "C Y" => 3 + 3,
                "C Z" => 6 + 1,
                _ => panic!("Unexpected")
            }
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
        assert_eq!(solution1(&input), 15);
    }

    #[test]
    fn test_solution2() {
        let input = read_file("test.txt");
        assert_eq!(solution2(&input), 12);
    }
}

fn main() {
    let input = read_file("input");
    let s1 = solution1(&input);
    println!("solution 1: {}", s1);

    let s2 = solution2(&input);
    println!("solution 2: {}", s2);
}
