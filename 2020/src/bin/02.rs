use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use regex::Regex;

struct Policy {
    min: usize,
    max: usize,
    character: char,
}

impl Policy {
    fn new(min: usize, max: usize, character: char) -> Self {
        Policy {
            min,
            max,
            character,
        }
    }

    fn validate_old(&self, password: &str) -> bool {
        let mut count = 0;
        for c in password.chars() {
            if c == self.character {
                count += 1
            }
        }
        count >= self.min && count <= self.max
    }

    fn validate_new(&self, password: &str) -> bool {
        let a = password.chars().nth(self.min - 1).unwrap_or('\0') == self.character;
        let b = password.chars().nth(self.max - 1).unwrap_or('\0') == self.character;
        a ^ b
    }
}

fn run(filename: &str) -> (usize, usize) {
    let re = Regex::new(r"(\d+)-(\d+) (\w+): (.+)").unwrap();
    let mut valid_old = 0;
    let mut valid_new = 0;
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let matches = re.captures(line.as_str()).unwrap();
        let policy = Policy::new(
            matches[1].parse().unwrap(),
            matches[2].parse().unwrap(),
            matches[3].parse().unwrap(),
        );
        if policy.validate_old(&matches[4]) {
            valid_old += 1;
        }
        if policy.validate_new(&matches[4]) {
            valid_new += 1;
        }
    }
    (valid_old, valid_new)
}

fn main() {
    println!("{:?}", run("input/02-example.txt"));
    println!("{:?}", run("input/02.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_02() {
        let (first, second) = super::run("input/02-example.txt");
        assert_eq!(first, 2);
        assert_eq!(second, 1);
    }

    #[test]
    fn test_input_02() {
        let (first, second) = super::run("input/02.txt");
        assert_eq!(first, 560);
        assert_eq!(second, 303);
    }
}
