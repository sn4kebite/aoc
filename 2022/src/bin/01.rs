use std::fs::File;
use std::io::{BufRead, BufReader};

fn run(filename: &str) -> (usize, usize) {
    let mut elves = vec![0];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            elves.push(0);
            continue;
        }
        let value: usize = line.parse().expect("invalid food value");
        if let Some(elf) = elves.last_mut() {
            *elf += value;
        }
    }
    elves.sort();
    elves.reverse();
    (elves[0], elves[0..3].iter().sum())
}

fn main() {
    println!("{:?}", run("input/01-example.txt"));
    println!("{:?}", run("input/01.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_01() {
        let (first, second) = super::run("input/01-example.txt");
        assert_eq!(first, 24000);
        assert_eq!(second, 45000);
    }

    #[test]
    fn test_01() {
        let (first, second) = super::run("input/01.txt");
        assert_eq!(first, 70374);
        assert_eq!(second, 204610);
    }
}
