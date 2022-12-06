use std::fs::File;
use std::io::prelude::*;

fn solution(input: &str, n: usize) -> usize {
    let bytes = input.as_bytes();
    for i in 0..=bytes.len() - n {
        let xs = &bytes[i..i + n];
        if xs.iter().enumerate().all(|(i, x)| !xs[i+1..].contains(x)) {
            return i + n
        }
    }
    0
}

fn solution1(input: &str) -> usize {
    solution(input, 4)
}

fn solution2(input: &str) -> usize {
    solution(input, 14)
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
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(solution1(&input), 7);

        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(solution1(&input), 5);

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(solution1(&input), 6);

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(solution1(&input), 10);

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(solution1(&input), 11);
    }

    #[test]
    fn test_solution2() {
        let input = "mjqjpqmgbljsphdztnvjfqwrcgsmlb";
        assert_eq!(solution2(&input), 19);

        let input = "bvwbjplbgvbhsrlpgdmjqwftvncz";
        assert_eq!(solution2(&input), 23);

        let input = "nppdvjthqldpwncqszvftbrmjlhg";
        assert_eq!(solution2(&input), 23);

        let input = "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg";
        assert_eq!(solution2(&input), 29);

        let input = "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw";
        assert_eq!(solution2(&input), 26);
    }
}

fn main() {
    let input = read_file("input");
    let s1 = solution1(&input);
    println!("solution 1: {}", s1);

    let s2 = solution2(&input);
    println!("solution 2: {}", s2);
}
