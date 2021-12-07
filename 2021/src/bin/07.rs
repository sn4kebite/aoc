use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn sum(n: i32) -> i32 {
    n * (n + 1) / 2
}

fn calc_fuel(filename: &str) -> (usize, usize) {
    let mut buf = String::new();
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    reader.read_line(&mut buf).expect("Cannot read input");
    let crabs: Vec<i32> = buf.trim().split(',').map(|s| s.parse().unwrap()).collect();
    let mut fuel = 0;
    let mut fuel2 = 0;
    let mut pos1 = 0;
    let mut pos2 = 0;
    for pos in *crabs.iter().min().unwrap()..*crabs.iter().max().unwrap() {
        let pos = pos as i32;
        let this: i32 = crabs.iter().map(|crab| (pos - *crab).abs()).sum();
        if fuel == 0 || this < fuel {
            fuel = this;
            pos1 = pos;
        }
        let this: i32 = crabs.iter().map(|crab| sum((pos - *crab).abs())).sum();
        if fuel2 == 0 || this < fuel2 {
            fuel2 = this;
            pos2 = pos;
        }
    }
    println!("pos1={} pos2={}", pos1, pos2);
    (fuel as usize, fuel2 as usize)
}

fn run(filename: &str) -> (usize, usize) {
    calc_fuel(filename)
}

fn main() {
    println!("{:?}", run("input/07-example.txt"));
    println!("{:?}", run("input/07.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_07() {
        let (first, second) = super::run("input/07-example.txt");
        assert_eq!(first, 37);
        assert_eq!(second, 168);
    }

    #[test]
    fn test_input_07() {
        let (first, second) = super::run("input/07.txt");
        assert_eq!(first, 355592);
        assert_eq!(second, 101618069);
    }
}
