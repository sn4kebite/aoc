use std::fs::File;
use std::io::{BufRead, BufReader};

const WORDS: &[&str] = &[
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut sum = 0;
    let mut sum2 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        // hold digit indices and values
        let mut digits: Vec<(usize, usize)> = line
            .char_indices()
            // Filter all characters that are valid base-10 digits
            .filter(|(_, c)| c.is_digit(10))
            // Transform filtered characters to usize
            .map(|(i, c)| (i, c.to_digit(10).expect("invalid digit") as usize))
            .collect();
        // ditto for words
        let words: Vec<(usize, usize)> = WORDS
            .iter()
            .enumerate()
            // Map each (index, word) and iterate over matches for that word in line
            // For each match, return the line index (i) and word number (word index + 1)
            .map(|(wi, w)| line.match_indices(w).map(move |(i, _)| (i, wi + 1)))
            // The result is a nested array so we need to flatten it
            .flatten()
            .collect();
        digits.sort_by_key(|e| e.0);
        if digits.len() > 0 {
            sum += digits.first().unwrap().1 * 10 + digits.last().unwrap().1;
        }

        // add words and resort
        digits.extend(words);
        digits.sort_by_key(|e| e.0);
        if digits.len() > 0 {
            sum2 += digits.first().unwrap().1 * 10 + digits.last().unwrap().1;
        }
    }
    (sum, sum2)
}

fn main() {
    println!("{:?}", run("input/01-example1.txt"));
    println!("{:?}", run("input/01-example2.txt"));
    println!("{:?}", run("input/01.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_01() {
        let (first, _) = super::run("input/01-example1.txt");
        assert_eq!(first, 142);
        //assert_eq!(second, 0);
    }

    #[test]
    fn test_example_02() {
        let (_, second) = super::run("input/01-example2.txt");
        //assert_eq!(first, 0);
        assert_eq!(second, 281);
    }

    #[test]
    fn test_01() {
        let (first, second) = super::run("input/01.txt");
        assert_eq!(first, 57346);
        assert_eq!(second, 57345);
    }
}
