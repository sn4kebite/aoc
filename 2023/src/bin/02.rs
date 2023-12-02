use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl CubeSet {
    pub fn new(red: usize, green: usize, blue: usize) -> Self {
        Self { red, green, blue }
    }

    pub fn parse(s: &str) -> Self {
        let mut r = 0;
        let mut g = 0;
        let mut b = 0;
        for cube in s.split(", ") {
            let (n, name) = cube.split_once(' ').expect("cannot split cube");
            let n: usize = n.parse().expect("invalid cube number");
            match name {
                "red" => r += n,
                "green" => g += n,
                "blue" => b += n,
                _ => panic!("invalid name {}", name),
            }
        }
        Self::new(r, g, b)
    }
}

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let limit_r = 12;
    let limit_g = 13;
    let limit_b = 14;
    let mut sum = 0;
    let mut sum2 = 0;
    for (id, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let (_, game) = line.split_once(": ").expect("cannot split line");
        let game: Vec<CubeSet> = game.split("; ").map(|s| CubeSet::parse(s)).collect();
        let max_r: usize = game.iter().map(|cube| cube.red).max().unwrap();
        let max_g: usize = game.iter().map(|cube| cube.green).max().unwrap();
        let max_b: usize = game.iter().map(|cube| cube.blue).max().unwrap();
        if max_r <= limit_r && max_g <= limit_g && max_b <= limit_b {
            sum += id + 1;
        }
        sum2 += max_r * max_g * max_b;
    }
    (sum, sum2)
}

fn main() {
    println!("{:?}", run("input/02-example.txt"));
    println!("{:?}", run("input/02.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_01() {
        let (first, second) = super::run("input/02-example.txt");
        assert_eq!(first, 8);
        assert_eq!(second, 2286);
    }

    #[test]
    fn test_02() {
        let (first, second) = super::run("input/02.txt");
        assert_eq!(first, 2913);
        assert_eq!(second, 55593);
    }
}
