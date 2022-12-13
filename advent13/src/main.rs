use std::fs::File;
use std::io::prelude::*;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq)]
enum Node {
    Value(u8),
    Nodes(Vec<Node>),
}

#[derive(Debug)]
struct Operands {
    left: Node,
    right: Node,
}

impl Node {
    fn from(line: &str) -> Self {
        let mut stack: Vec<Vec<Node>> = vec![];
        let mut nodes: Vec<Node> = vec![];

        let mut maybe_value: Option<u8> = None;
        for c in line.chars() {
            if c.is_ascii_digit() {
                let d = c.to_digit(10).unwrap() as u8;
                if let Some(v) = maybe_value {
                    maybe_value = Some(v * 10 + d)
                } else {
                    maybe_value = Some(d);
                }
            } else {
                match c {
                    '[' => {
                        stack.push(nodes);
                        nodes = vec![];
                    },
                    ',' => {
                        if let Some(v) = maybe_value {
                            nodes.push(Node::Value(v));
                        }
                    },
                    ']' => {
                        if let Some(v) = maybe_value {
                            nodes.push(Node::Value(v));
                        }

                        let mut parent = stack.pop().unwrap();
                        parent.push(Node::Nodes(nodes));
                        nodes = parent;
                    },
                    _ => {
                        panic!("Unexpected char: {}", c);
                    },
                }
                maybe_value = None;
            }
        }
        nodes.pop().unwrap()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        match self {
            Self::Value(l) => {
                match other {
                    Self::Value(r) => l.cmp(r),
                    Self::Nodes(_) => {
                        Self::Nodes(vec![Self::Value(*l)]).cmp(other)
                    },
                }
            },
            Self::Nodes(l) => {
                match other {
                    Self::Value(v) => {
                        self.cmp(&Self::Nodes(vec![Self::Value(*v)]))
                    },
                    Self::Nodes(r) => {
                        let ord = l.iter().zip(r.iter()).find_map(|(l, r)| {
                                let ord = l.cmp(r);
                                if ord == Ordering::Equal {
                                    None
                                } else {
                                    Some(ord)
                                }
                            }).unwrap_or(Ordering::Equal);
                        if ord == Ordering::Equal {
                            l.len().cmp(&r.len())
                        } else {
                            ord
                        }
                    }
                }
            },
        }
    }
}

fn solution1(input: &str) -> usize {
    input
        .trim_end()
        .split("\n\n")
        .map(|line| {
            let mut it = line.split('\n');
            Operands {
                left: Node::from(it.next().unwrap()),
                right: Node::from(it.next().unwrap()),
            }
        })
        .enumerate()
        .filter(|(_i, operands)| operands.left < operands.right)
        .map(|(i, _)| i + 1)
        .sum()
}

fn divs() -> [Node; 2] {
    [
        Node::Nodes(vec![
            Node::Nodes(vec![
                Node::Value(2)
            ])
        ]),
        Node::Nodes(vec![
            Node::Nodes(vec![
                Node::Value(6)
            ])
        ]),
    ]
}

fn solution2(input: &str) -> usize {
    let mut nodes: Vec<Node> = input
        .trim_end()
        .split('\n')
        .filter(|line| !line.is_empty())
        .map(|line| {
            Node::from(line)
        })
        .collect();

    for div in divs() {
        nodes.push(div);
    }

    nodes.sort();

    divs().iter().map(|x| {
        nodes.iter().position(|y| x == y).unwrap() + 1
    }).product()
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
    fn test_parse() {
        assert_eq!(Node::from("[1,2]"), Node::Nodes(vec![
                Node::Value(1),
                Node::Value(2),
        ]));
        assert_eq!(Node::from("[1,10,2]"), Node::Nodes(vec![
                Node::Value(1),
                Node::Value(10),
                Node::Value(2),
        ]));
        assert_eq!(Node::from("[[1]]"), Node::Nodes(vec![
                Node::Nodes(vec![
                    Node::Value(1)
                ])
        ]));
        assert_eq!(Node::from("[[1],[2]]"), Node::Nodes(vec![
                Node::Nodes(vec![
                    Node::Value(1)
                ]),
                Node::Nodes(vec![
                    Node::Value(2)
                ]),
        ]));
        assert_eq!(Node::from("[[1],2]"), Node::Nodes(vec![
                Node::Nodes(vec![
                    Node::Value(1)
                ]),
                Node::Value(2),
        ]));
    }

    #[test]
    fn test_solution1() {
        let input = read_file("test.txt");
        assert_eq!(solution1(&input), 13);
    }

    #[test]
    fn test_solution2() {
        let input = read_file("test.txt");
        assert_eq!(solution2(&input), 140);
    }
}

fn main() {
    let input = read_file("input");
    let s1 = solution1(&input);
    println!("solution 1: {}", s1);

    let s2 = solution2(&input);
    println!("solution 2: {}", s2);
}
