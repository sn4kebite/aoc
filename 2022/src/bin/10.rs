use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Op {
    Addx(isize),
    Noop,
}

impl Op {
    pub fn parse(s: &str) -> Option<Self> {
        if s == "noop" {
            Some(Op::Noop)
        } else if s.starts_with("addx ") {
            let (_, v) = s.split_once(' ').unwrap();
            Some(Op::Addx(v.parse().unwrap()))
        } else {
            None
        }
    }

    pub fn cycles(&self) -> usize {
        match self {
            Op::Addx(_) => 2,
            Op::Noop => 1,
        }
    }
}

fn run(filename: &str) -> (usize, [bool; 40 * 6]) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut x = 1;
    let mut signal = 0;
    let mut cycle = 0;
    let mut next_cycle = 20;
    let mut bitmap = [false; 40 * 6];
    for line in reader.lines() {
        let line = line.unwrap();
        let op = Op::parse(line.as_str()).unwrap();
        for _ in 0..op.cycles() {
            let c = cycle % 40;
            if c >= x - 1 && c <= x + 1 {
                bitmap[cycle as usize] = true;
            }
            cycle += 1;
            if cycle == next_cycle {
                signal += cycle * x;
                next_cycle += 40;
            }
        }
        match op {
            Op::Addx(v) => x = x + v,
            Op::Noop => (),
        }
    }
    for y in 0..6 {
        for x in 0..40 {
            let c = if bitmap[y * 40 + x] { '#' } else { ' ' };
            print!("{}", c);
        }
        println!();
    }
    (signal as usize, bitmap)
}

fn main() {
    println!("{:?}", run("input/10-example.txt"));
    println!("{:?}", run("input/10.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_10() {
        let (first, second) = super::run("input/10-example.txt");
        assert_eq!(first, 13140);
        assert_eq!(
            second,
            [
                true, true, false, false, true, true, false, false, true, true, false, false, true,
                true, false, false, true, true, false, false, true, true, false, false, true, true,
                false, false, true, true, false, false, true, true, false, false, true, true,
                false, false, true, true, true, false, false, false, true, true, true, false,
                false, false, true, true, true, false, false, false, true, true, true, false,
                false, false, true, true, true, false, false, false, true, true, true, false,
                false, false, true, true, true, false, true, true, true, true, false, false, false,
                false, true, true, true, true, false, false, false, false, true, true, true, true,
                false, false, false, false, true, true, true, true, false, false, false, false,
                true, true, true, true, false, false, false, false, true, true, true, true, true,
                false, false, false, false, false, true, true, true, true, true, false, false,
                false, false, false, true, true, true, true, true, false, false, false, false,
                false, true, true, true, true, true, false, false, false, false, false, true, true,
                true, true, true, true, false, false, false, false, false, false, true, true, true,
                true, true, true, false, false, false, false, false, false, true, true, true, true,
                true, true, false, false, false, false, false, false, true, true, true, true, true,
                true, true, true, true, true, true, false, false, false, false, false, false,
                false, true, true, true, true, true, true, true, false, false, false, false, false,
                false, false, true, true, true, true, true, true, true, false, false, false, false,
                false
            ]
        );
    }

    #[test]
    fn test_10() {
        let (first, second) = super::run("input/10.txt");
        assert_eq!(first, 12560);
        assert_eq!(
            second,
            [
                true, true, true, false, false, true, false, false, false, false, true, true, true,
                false, false, false, true, true, false, false, true, true, true, true, false, true,
                true, true, false, false, false, true, true, false, false, true, false, false,
                false, false, true, false, false, true, false, true, false, false, false, false,
                true, false, false, true, false, true, false, false, true, false, true, false,
                false, false, false, true, false, false, true, false, true, false, false, true,
                false, true, false, false, false, false, true, false, false, true, false, true,
                false, false, false, false, true, false, false, true, false, true, false, false,
                true, false, true, true, true, false, false, true, true, true, false, false, true,
                false, false, false, false, true, false, false, false, false, true, true, true,
                false, false, true, false, false, false, false, true, true, true, false, false,
                true, true, true, true, false, true, false, false, false, false, true, false,
                false, true, false, true, false, false, false, false, true, false, false, false,
                false, true, false, false, false, false, true, false, false, false, false, true,
                false, false, false, false, true, false, false, true, false, true, false, false,
                false, false, true, false, false, true, false, true, false, false, true, false,
                true, false, false, false, false, true, false, false, false, false, true, true,
                true, true, false, true, false, false, false, false, true, false, false, true,
                false, true, false, false, false, false, true, true, true, false, false, false,
                true, true, false, false, true, true, true, true, false
            ]
        );
    }
}
