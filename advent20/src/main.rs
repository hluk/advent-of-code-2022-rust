use std::fs::File;
use std::io::prelude::*;

type Num = i64;
const DECRYPTION_KEY: Num = 811589153;

fn parse(input: &str) -> Vec<Num> {
    input.trim_end()
        .split('\n')
        .map(|x| x.parse().unwrap())
        .collect()
}

fn mix(i0: usize, ns: &Vec<Num>, is: &mut [usize]) {
    let mut n = ns[i0];
    let i = is.iter().copied().position(|j| j == i0).unwrap();
    let l = ns.len() as Num;

    // Skip the current n in repeated loops.
    let d = n / (l - 1);

    if n < 0 {
        n = l * (d.abs() + 1) + n - 1;
    }

    let j = ((n + d + i as Num) % l) as usize;
    if i < j {
        for k in i..j {
            is.swap(k, k + 1);
        }
    } else {
        for k in (j+2..i+1).rev() {
            is.swap(k, k - 1);
        }
    }
}

fn groove_coords(ns: &[Num], is: &[usize]) -> Num {
    let z0 = ns.iter().copied().position(|n| n == 0).unwrap();
    let z = is.iter().copied().position(|j| j == z0).unwrap();
    let l = ns.len();
    [1000, 2000, 3000].into_iter()
        .map(|i| ns[is[(z + i) % l]]).sum()
}

fn solution1(input: &str) -> Num {
    let ns = parse(input);
    let l = ns.len();
    let mut is: Vec<usize> = (0..l).collect();
    for i0 in 0..l {
        mix(i0, &ns, &mut is);
    }
    groove_coords(&ns, &is)
}

fn solution2(input: &str) -> Num {
    let ns: Vec<Num> = parse(input).iter().map(|n| n * DECRYPTION_KEY).collect();
    let l = ns.len();
    let mut is: Vec<usize> = (0..l).collect();
    for _ in 0..10 {
        for i0 in 0..l {
            mix(i0, &ns, &mut is);
        }
    }
    groove_coords(&ns, &is)
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
        assert_eq!(solution1(&input), 3);
    }

    #[test]
    fn test_solution2() {
        let input = read_file("test.txt");
        assert_eq!(solution2(&input), 1623178306);
    }

    #[test]
    fn test_full_solution1() {
        let input = read_file("input");
        assert_eq!(solution1(&input), 7153);
    }

    #[test]
    fn test_full_solution2() {
        let input = read_file("input");
        assert_eq!(solution2(&input), 6146976244822);
    }
}

fn main() {
    let input = read_file("input");

    let s1 = solution1(&input);
    println!("solution 1: {}", s1);

    let s2 = solution2(&input);
    println!("solution 2: {}", s2);
}
