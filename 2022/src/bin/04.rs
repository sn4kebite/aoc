use std::fs::File;
use std::io::{BufRead, BufReader};

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut contains = 0;
    let mut overlaps = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let (a, b) = line.split_once(',').unwrap();
        let (a1, a2) = a.split_once('-').unwrap();
        let (b1, b2) = b.split_once('-').unwrap();
        let a1 = a1.parse::<usize>().unwrap();
        let a2 = a2.parse::<usize>().unwrap();
        let b1 = b1.parse::<usize>().unwrap();
        let b2 = b2.parse::<usize>().unwrap();
        // range is contained
        if (b1 >= a1 && b2 <= a2) || (a1 >= b1 && a2 <= b2) {
            contains += 1;
            overlaps += 1;
        // range is overlapping
        } else if (b1 <= a1 && b2 >= a1) || (b1 <= a2 && b2 >= a2) {
            overlaps += 1;
        }
    }
    (contains, overlaps)
}

fn main() {
    println!("{:?}", run("input/04-example.txt"));
    println!("{:?}", run("input/04.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_04() {
        let (first, second) = super::run("input/04-example.txt");
        assert_eq!(first, 2);
        assert_eq!(second, 4);
    }

    #[test]
    fn test_04() {
        let (first, second) = super::run("input/04.txt");
        assert_eq!(first, 657);
        assert_eq!(second, 938);
    }
}
