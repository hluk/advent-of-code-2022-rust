use std::fs::File;
use std::io::prelude::*;
use std::collections::HashSet;
use std::fmt;
use std::cmp;

#[derive(Clone, Copy, Eq, PartialEq, PartialOrd, Hash)]
struct Pos {
    x: i64,
    y: i64,
}

impl Pos {
    fn new() -> Self {
        Pos {x: 0, y: 0}
    }

    fn add(self: &mut Self, dx: i64, dy: i64) {
        self.x += dx;
        self.y += dy;
    }
}

impl fmt::Debug for Pos {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{},{}]\n", self.x, self.y)
    }
}

impl cmp::Ord for Pos {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        (self.x, self.y).cmp(&(other.x, other.y))
    }
}

struct Rope {
    knots: Vec<Pos>,
    visited: HashSet<Pos>,
}

impl Rope {
    fn new(n: usize) -> Self {
        Rope {knots: vec![Pos::new(); n], visited: HashSet::new()}
    }
}

impl fmt::Debug for Rope {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Rope {{\n")?;

        write!(f, "  knots:   {{")?;
        for v in &self.knots {
            write!(f, " {},{} ", v.x, v.y)?;
        }
        write!(f, "}}\n")?;

        write!(f, "  visited: {{")?;
        let mut vv: Vec<Pos> = self.visited.iter().cloned().collect();
        vv.sort();
        for v in &vv {
            write!(f, " {},{} ", v.x, v.y)?;
        }
        write!(f, "}}\n")?;

        write!(f, "}}")
    }
}

fn move_head(rope: &mut Rope, x: i64, y: i64) {
    let dx = x.signum();
    let dy = y.signum();
    let steps = x.abs().max(y.abs());
    for _i in 0..steps {
        rope.knots[0].add(dx, dy);
        for i in 0..rope.knots.len()-1 {
            let dx = rope.knots[i].x - rope.knots[i + 1].x;
            if dx.abs() > 1 {
                let dy = rope.knots[i].y - rope.knots[i + 1].y;
                rope.knots[i + 1].add(dx.signum(), dy.signum());
            }
            let dy = rope.knots[i].y - rope.knots[i + 1].y;
            if dy.abs() > 1 {
                let dx = rope.knots[i].x - rope.knots[i + 1].x;
                rope.knots[i + 1].add(dx.signum(), dy.signum());
            }
        }
        rope.visited.insert(rope.knots[rope.knots.len() - 1]);
    }
}

fn solution(input: &str, n: usize) -> usize {
    let mut rope = Rope::new(n);
    rope.visited.insert(rope.knots[rope.knots.len() - 1]);
    input
        .trim_end()
        .split('\n')
        .for_each(|line| {
            let mut it = line.split(' ');
            let dir = it.next().unwrap();
            let n = it.next().unwrap().parse::<i64>().unwrap();
            match dir {
                "U" => move_head(&mut rope, 0, n),
                "D" => move_head(&mut rope, 0, -n),
                "R" => move_head(&mut rope, n, 0),
                "L" => move_head(&mut rope, -n, 0),
                _ => panic!("Unexpected")
            };
        });
    rope.visited.len()
}

fn solution1(input: &str) -> usize {
    solution(input, 2)
}

fn solution2(input: &str) -> usize {
    solution(input, 10)
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
        assert_eq!(solution1(&input), 13);
    }

    #[test]
    fn test_solution2() {
        let input = read_file("test.txt");
        assert_eq!(solution2(&input), 1);

        let input = read_file("test2.txt");
        assert_eq!(solution2(&input), 36);
    }
}

fn main() {
    let input = read_file("input");
    let s1 = solution1(&input);
    println!("solution 1: {}", s1);

    let s2 = solution2(&input);
    println!("solution 2: {}", s2);
}
