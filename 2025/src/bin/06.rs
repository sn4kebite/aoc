use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
enum Operator {
    Plus,
    Multiply,
}

impl Operator {
    pub fn parse(op: &str) -> Option<Self> {
        match op {
            "+" => Some(Operator::Plus),
            "*" => Some(Operator::Multiply),
            _ => None,
        }
    }

    pub fn exec(&self, a: usize, b: usize) -> usize {
        match self {
            Operator::Plus => a + b,
            Operator::Multiply => a * b,
        }
    }

    pub fn initial(&self) -> usize {
        match self {
            Operator::Plus => 0,
            Operator::Multiply => 1,
        }
    }
}

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let ops: Vec<Operator> = lines
        .pop()
        .unwrap()
        .split_whitespace()
        .map(|op| Operator::parse(op).unwrap())
        .collect();
    let numbers: Vec<Vec<usize>> = lines
        .iter()
        .map(|line| {
            line.trim()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();
    let mut numbers2 = vec![];
    let mut temp = vec![];
    for i in 0..lines[0].len() {
        let mut s = String::new();
        for line in &lines {
            let c = line.chars().nth(i).unwrap();
            if c != ' ' {
                s.push(line.chars().nth(i).unwrap());
            }
        }
        if s.is_empty() {
            numbers2.push(temp.clone());
            temp.clear();
        } else {
            temp.push(s.parse::<usize>().unwrap());
        }
    }
    numbers2.push(temp.clone());
    temp.clear();
    let mut grand_total = 0;
    for (i, op) in ops.iter().enumerate() {
        let value = numbers.iter().fold(op.initial(), |a, v| op.exec(a, v[i]));
        grand_total += value;
    }
    let mut grand_total2 = 0;
    for (i, op) in ops.iter().enumerate() {
        let value = numbers2[i].iter().fold(op.initial(), |a, v| op.exec(a, *v));
        grand_total2 += value;
    }
    (grand_total, grand_total2)
}

fn main() {
    println!("{:?}", run("input/06-example.txt"));
    println!("{:?}", run("input/06.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_06() {
        let (first, second) = super::run("input/06-example.txt");
        assert_eq!(first, 4277556);
        assert_eq!(second, 3263827);
    }

    #[test]
    fn test_06() {
        let (first, second) = super::run("input/06.txt");
        assert_eq!(first, 5552221122013);
        assert_eq!(second, 11371597126232);
    }
}
