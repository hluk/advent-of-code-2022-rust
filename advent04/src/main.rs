use std::fs::File;
use std::io::prelude::*;

fn count<Filter: Fn(&Vec<u32>) -> bool>(input: &str, filter: Filter) -> usize {
    input
        .split("\n")
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(|x| x == ',' || x == '-')
                .map(|x| x.parse::<u32>().unwrap())
                .collect()
        })
        .filter(filter)
        .count()
}

fn solution1(input: &str) -> usize {
    count(input, |x| {
        (x[0] >= x[2] && x[1] <= x[3]) ||
        (x[2] >= x[0] && x[3] <= x[1])
    })
}

fn solution2(input: &str) -> usize {
    count(input, |x| {
        (x[0] >= x[2] && x[0] <= x[3]) ||
        (x[1] >= x[2] && x[1] <= x[3]) ||
        (x[2] >= x[0] && x[2] <= x[1]) ||
        (x[2] >= x[0] && x[2] <= x[1])
    })
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
        assert_eq!(solution1(&input), 2);
    }

    #[test]
    fn test_solution2() {
        let input = read_file("test.txt");
        assert_eq!(solution2(&input), 4);
    }
}

fn main() {
    let input = read_file("input");
    let s1 = solution1(&input);
    println!("solution 1: {}", s1);

    let s2 = solution2(&input);
    println!("solution 2: {}", s2);
}
