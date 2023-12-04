use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0;
    let mut cards = HashMap::new();
    for (card, line) in reader.lines().enumerate() {
        let self_copies = *cards.entry(card).or_insert(1);
        let line = line.unwrap();
        let (winning, numbers) = line.split_once(": ").unwrap().1.split_once(" | ").unwrap();
        let winning: HashSet<usize> = winning
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let numbers: HashSet<usize> = numbers
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let matches: HashSet<usize> = winning.intersection(&numbers).map(|n| *n).collect();
        if !matches.is_empty() {
            sum += 1 * (2_usize).pow(matches.len() as u32 - 1);
            for i in 0..matches.len() {
                let i = i + card + 1;
                cards
                    .entry(i)
                    .and_modify(|e| *e += self_copies)
                    .or_insert(1 + self_copies);
            }
        }
    }
    (sum, cards.values().sum())
}

fn main() {
    println!("{:?}", run("input/04-example.txt"));
    println!("{:?}", run("input/04.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_01() {
        let (first, second) = super::run("input/04-example.txt");
        assert_eq!(first, 13);
        assert_eq!(second, 30);
    }

    #[test]
    fn test_04() {
        let (first, second) = super::run("input/04.txt");
        assert_eq!(first, 21485);
        assert_eq!(second, 11024379);
    }
}
