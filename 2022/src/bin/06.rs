use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn _run(filename: &str, marker_size: usize) -> usize {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let line: Vec<char> = line.trim().chars().collect();
    let mut first = 0;
    for i in marker_size..line.len() {
        let hash: HashSet<char> = HashSet::from_iter(line[i - marker_size..i].iter().map(|c| *c));
        if hash.len() == marker_size {
            first = i;
            break;
        }
    }
    first
}

fn run(filename: &str) -> (usize, usize) {
    (_run(filename, 4), _run(filename, 14))
}

fn main() {
    println!("{:?}", run("input/06-example1.txt"));
    println!("{:?}", run("input/06-example2.txt"));
    println!("{:?}", run("input/06-example3.txt"));
    println!("{:?}", run("input/06-example4.txt"));
    println!("{:?}", run("input/06-example5.txt"));
    println!("{:?}", run("input/06.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example1_06() {
        let (first, second) = super::run("input/06-example1.txt");
        assert_eq!(first, 7);
        assert_eq!(second, 19);
    }

    #[test]
    fn test_example2_06() {
        let (first, second) = super::run("input/06-example2.txt");
        assert_eq!(first, 5);
        assert_eq!(second, 23);
    }

    #[test]
    fn test_example3_06() {
        let (first, second) = super::run("input/06-example3.txt");
        assert_eq!(first, 6);
        assert_eq!(second, 23);
    }

    #[test]
    fn test_example4_06() {
        let (first, second) = super::run("input/06-example4.txt");
        assert_eq!(first, 10);
        assert_eq!(second, 29);
    }

    #[test]
    fn test_example5_06() {
        let (first, second) = super::run("input/06-example5.txt");
        assert_eq!(first, 11);
        assert_eq!(second, 26);
    }

    #[test]
    fn test_06() {
        let (first, second) = super::run("input/06.txt");
        assert_eq!(first, 1175);
        assert_eq!(second, 3217);
    }
}
