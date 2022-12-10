use std::fs::File;
use std::io::prelude::*;

const CYCLES: &[usize] = &[20, 60, 100, 140, 180, 220];

fn get_xs(input: &str) -> Vec<i64> {
    let mut x = 1;
    input
        .trim_end()
        .split('\n')
        .flat_map(|line| {
            let mut it = line.split(' ');
            if it.next().unwrap() == "addx" {
                let y = x;
                x += it.next().unwrap().parse::<i64>().unwrap();
                vec![y, y]
            } else {
                vec![x]
            }
        }).collect()
}

fn solution1(input: &str) -> i64 {
    let xs = get_xs(input);
    CYCLES.iter()
        .map(|cycle| *cycle as i64 * xs[*cycle - 1])
        .sum()
}

fn solution2(input: &str) -> String {
    get_xs(input)
        .iter()
        .enumerate()
        .map(|(pixel, x)| {
            let col = pixel % 40;
            if col as i64 >= x - 1 && col as i64 <= x + 1 {
                if col < 39 { "#" } else { "#\n" }
            } else if col < 39 { "." } else { ".\n" }
        }).collect()
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
        assert_eq!(solution1(&input), 13140);
    }

    #[test]
    fn test_solution2() {
        let input = read_file("test.txt");
        assert_eq!(
            solution2(&input), "\
            ##..##..##..##..##..##..##..##..##..##..\n\
            ###...###...###...###...###...###...###.\n\
            ####....####....####....####....####....\n\
            #####.....#####.....#####.....#####.....\n\
            ######......######......######......####\n\
            #######.......#######.......#######.....\n\
            "
        );
    }
}

fn main() {
    let input = read_file("input");
    let s1 = solution1(&input);
    println!("solution 1: {}", s1);

    let s2 = solution2(&input);
    println!("solution 2:\n{}", s2);
}
