use std::fs::File;
use std::io::prelude::*;

type Pos = (i32, i32);
struct Sensor {
    pos: Pos,
    distance: i32,
}

impl Sensor {
    fn from(line: &str) -> Self {
        let mut it = line
            .split(&['=', ',', ':'][..])
            .skip(1)
            .step_by(2)
            .map(|x| x.parse::<i32>().unwrap());
        let pos = (it.next().unwrap(), it.next().unwrap());
        let beacon = (it.next().unwrap(), it.next().unwrap());
        let distance = manhattan(pos, beacon) as i32;
        Sensor {pos, distance}
    }
}

fn parse(input: &str) -> Vec<Sensor> {
    input
        .trim_end()
        .split('\n')
        .map(Sensor::from)
        .collect()
}

fn manhattan(a: Pos, b: Pos) -> u32 {
    a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
}

fn solution1(input: &str, row: i32) -> usize {
    let sensors = parse(input);
    let xs = sensors.iter().fold((i32::MAX, i32::MIN), |xs, sensor| {
        let d_row = sensor.pos.1.abs_diff(row) as i32;
        let d = sensor.distance - d_row;
        if d <= 0 {
            xs
        } else {
            (xs.0.min(sensor.pos.0 - d), xs.1.max(sensor.pos.0 + d))
        }
    });
    (xs.1 - xs.0) as usize
}

fn solution2(input: &str, max: i32) -> i64 {
    let sensors = parse(input);
    for sensor in &sensors {
        let y1 = (sensor.pos.1 - sensor.distance).max(0);
        let y2 = (sensor.pos.1 + sensor.distance).min(max);
        for y in y1..y2 {
            let d_row = sensor.pos.1.abs_diff(y) as i32;
            let d = sensor.distance - d_row;
            if d > 0 {
                let x = sensor.pos.0 + d + 1;
                if x < 0 || x > max { continue; }
                let intersects = sensors.iter().any(|p| {
                    let d_row = p.pos.1.abs_diff(y) as i32;
                    let d = p.distance - d_row;
                    d > 0 && p.pos.0 - d <= x && x <= p.pos.0 + d
                });
                if !intersects {
                    return x as i64 * 4000000 + y as i64;
                }
            }
        }
    }
    0
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
        assert_eq!(solution1(&input, 10), 26);
    }

    #[test]
    fn test_solution2() {
        let input = read_file("test.txt");
        assert_eq!(solution2(&input, 20), 56000011);
    }
}

fn main() {
    let input = read_file("input");

    let s1 = solution1(&input, 2000000);
    println!("solution 1: {}", s1);

    let s2 = solution2(&input, 4000000);
    println!("solution 2: {}", s2);
}
