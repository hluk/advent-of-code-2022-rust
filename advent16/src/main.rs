use std::cmp::Ordering;
use std::fs::File;
use std::io::prelude::*;
use std::collections::BinaryHeap;
use std::collections::HashMap;
use std::hash::Hash;

type ValveId = u8;
struct Valve {
    rate: ValveId,
    next: Vec<ValveId>,
}
type Valves = HashMap<ValveId, Valve>;

fn parse(input: &str) -> (u8, Valves) {
    let mut valve_ids = HashMap::<&str, u8>::new();
    let mut free_valve_id = 0;
    let valves_it = input
        .trim_end()
        .split('\n')
        .map(|line| {
            // line: "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB"
            let mut it = line.split(&[' ', '=', ';', ','][..]);
            let valve_id = it.nth(1).unwrap();
            let valve = *valve_ids.entry(valve_id)
                .or_insert_with(|| {free_valve_id += 1; free_valve_id});
            let rate = it.nth(3).unwrap().parse::<u8>().unwrap();
            let next = it.skip(5).step_by(2).map(|id| {
                *valve_ids.entry(id)
                    .or_insert_with(|| {free_valve_id += 1; free_valve_id})
            }).collect();
            (valve, Valve {rate, next})
        });
    let valves = HashMap::from_iter(valves_it);
    (valve_ids["AA"], valves)
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct Flowing {
    flowing: u64,
}

impl Flowing {
    fn none() -> Self {
        Flowing {flowing: 0}
    }

    fn is_open(&self, valve_id: u8) -> bool {
        (self.flowing & (1 << valve_id)) > 0
    }

    fn with_opened(&self, valve_id: u8) -> Flowing {
        Flowing { flowing: (self.flowing | (1 << valve_id)) }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct State {
    flow: usize,
    remaining_time: usize,
    flowing: Flowing,
    opened: usize,
}

impl State {
    fn new(remaining_time: usize) -> Self {
        State {
            flow: 0,
            remaining_time,
            flowing: Flowing::none(),
            opened: 0,
        }
    }

    fn cost(&self) -> usize {
        self.flow + (16 - self.opened) * 25 * self.remaining_time / 2
    }
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost().cmp(&other.cost())
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Visited<Key> {
    visited: HashMap<(Key, u64), (usize, usize)>,
    heap: BinaryHeap<(State, Key)>,
}

impl<Key> Visited<Key> {
    fn new() -> Self
        where Key: Ord
    {
        let visited = HashMap::<(Key, u64), (usize, usize)>::new();
        let heap = BinaryHeap::new();
        Visited {visited, heap}
    }

    fn visit(&mut self, key: Key, state: &State)
        where Key: Copy + Eq + Hash + Ord
    {
        let v = (key, state.flowing.flowing);
        if let Some((time, flow)) = self.visited.get(&v).copied() {
            if time <= state.remaining_time && flow >= state.flow {
                return;
            }
        }
        self.visited.insert(v, (state.remaining_time, state.flow));
        self.heap.push((*state, key));
    }
}

fn solution1(input: &str) -> usize {
    let (start, valves) = parse(input);
    let mut visited = Visited::<u8>::new();
    let start_state = State::new(30);
    visited.visit(start, &start_state);

    let mut max_flow = 0;
    while let Some((state, id)) = visited.heap.pop() {
        let valve = &valves[&id];
        if valve.rate > 0 && !state.flowing.is_open(id) && state.remaining_time > 1 {
            let new = State {
                flow: (state.remaining_time - 1) * valve.rate as usize,
                remaining_time: 1,
                flowing: state.flowing.with_opened(id),
                opened: 1,
            };
            visited.visit(id, &new);
        }

        for next_id in &valve.next {
            let next = &valves[next_id];

            if next.rate > 0 && !state.flowing.is_open(*next_id) && state.remaining_time > 2 {
                let flow = state.flow + (state.remaining_time - 2) * next.rate as usize;
                max_flow = max_flow.max(flow);

                if state.remaining_time > 3 {
                    let new = State {
                        flow,
                        remaining_time: state.remaining_time - 2,
                        flowing: state.flowing.with_opened(*next_id),
                        opened: state.opened + 1,
                    };
                    visited.visit(*next_id, &new);
                }
            }

            if state.remaining_time > 2 {
                let new = State {
                    flow: state.flow,
                    remaining_time: state.remaining_time - 1,
                    flowing: state.flowing,
                    opened: state.opened,
                };
                visited.visit(*next_id, &new);
            }
        }
    }

    max_flow
}

fn solution2(input: &str) -> usize {
    let (start, valves) = parse(input);
    let max_opened = valves.iter().filter(|(_, v)| v.rate > 0).count();
    let mut visited = Visited::<(u8, u8)>::new();
    let start_state = State::new(26);
    visited.visit((start, start), &start_state);

    let mut i = 0;
    let mut max_flow = 0;
    while let Some((state, (id1, id2))) = visited.heap.pop() {
        if i > 4000000 { break; }
        i += 1;

        if state.opened == max_opened { continue; }
        if state.remaining_time <= 1 { continue; }

        let mut s1 = Vec::<(State, u8)>::new();
        let mut s2 = Vec::<(State, u8)>::new();

        for (id, s) in [(id1, &mut s1), (id2, &mut s2)] {
            let valve = &valves[&id];
            if valve.rate > 0 && !state.flowing.is_open(id) {
                let new = State {
                    flow: (state.remaining_time - 1) * valve.rate as usize,
                    remaining_time: 1,
                    flowing: state.flowing.with_opened(id),
                    opened: 1,
                };
                s.push((new, id));
            }

            for next_id in &valves[&id].next {
                let new = State {
                    flow: 0,
                    remaining_time: 1,
                    flowing: state.flowing,
                    opened: 0,
                };
                s.push((new, *next_id));
            }
        }

        for (u, id1) in &s1 {
            for (v, id2) in &s2 {
                // Both cannot open same valve.
                if id1 == id2 && u.opened + v.opened == 2 {
                    continue;
                }

                let flow = state.flow + u.flow + v.flow;
                if flow > max_flow {
                    max_flow = flow;
                    println!("Current max flow: {}", flow);
                    i = 0;
                };

                if state.remaining_time > 2 {
                    let new = State {
                        flow: flow,
                        remaining_time: state.remaining_time - 1,
                        flowing: Flowing {
                            flowing: u.flowing.flowing | v.flowing.flowing
                        },
                        opened: state.opened + u.opened + v.opened,
                    };

                    let id = if id1 < id2 { (*id1, *id2) } else { (*id2, *id1) };
                    visited.visit(id, &new);
                }
            }
        }
    }

    max_flow
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
    fn test_flowing() {
        assert_eq!(Flowing::none().is_open(1), false);
        assert_eq!(Flowing::none().is_open(2), false);
        assert_eq!(Flowing::none().is_open(63), false);

        assert_eq!(Flowing::none().with_opened(1).is_open(1), true);
        assert_eq!(Flowing::none().with_opened(1).is_open(2), false);
        assert_eq!(Flowing::none().with_opened(1).is_open(3), false);
        assert_eq!(Flowing::none().with_opened(1).is_open(4), false);

        assert_eq!(Flowing::none().with_opened(2).is_open(1), false);
        assert_eq!(Flowing::none().with_opened(2).is_open(2), true);
        assert_eq!(Flowing::none().with_opened(2).is_open(3), false);
        assert_eq!(Flowing::none().with_opened(2).is_open(4), false);

        assert_eq!(Flowing::none().with_opened(3).is_open(1), false);
        assert_eq!(Flowing::none().with_opened(3).is_open(2), false);
        assert_eq!(Flowing::none().with_opened(3).is_open(3), true);
        assert_eq!(Flowing::none().with_opened(3).is_open(4), false);

        assert_eq!(Flowing::none().with_opened(63).is_open(63), true);
        assert_eq!(Flowing::none().with_opened(63).is_open(62), false);
        assert_eq!(Flowing::none().with_opened(63).is_open(1), false);
        assert_eq!((1..64).filter(
                |v| Flowing::none().with_opened(63).is_open(*v)).collect::<Vec<u8>>(), [63]);
    }

    #[test]
    fn test_solution1() {
        let input = read_file("test.txt");
        assert_eq!(solution1(&input), 1651);
    }

    #[test]
    fn test_solution2() {
        let input = read_file("test.txt");
        assert_eq!(solution2(&input), 1707);
    }
}

fn main() {
    let input = read_file("input");

    let s1 = solution1(&input);
    println!("solution 1: {}", s1);

    let s2 = solution2(&input);
    println!("solution 2: {}", s2);
}
