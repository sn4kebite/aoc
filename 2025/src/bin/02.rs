use std::fs::File;
use std::io::{BufRead, BufReader};

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).unwrap();
    let ranges: Vec<(usize, usize)> = line
        .trim()
        .split(',')
        .map(|s| {
            let (a, b) = s.split_once('-').unwrap();
            (a.parse().unwrap(), b.parse().unwrap())
        })
        .collect();
    let mut invalid = 0;
    let mut invalid2 = 0;
    for (a, b) in &ranges {
        for id in *a..*b + 1 {
            let s = id.to_string();
            if s[0..s.len() / 2] == s[s.len() / 2..] {
                invalid += id;
            }
            for size in 1..s.len() {
                let mut equal = true;
                for start in (0..s.len() - size).step_by(size) {
                    if start + size > s.len() || start + size + size > s.len() {
                        equal = false;
                        break;
                    }
                    if s[start..start + size] != s[start + size..start + size + size] {
                        equal = false;
                        break;
                    }
                }
                if equal {
                    invalid2 += id;
                    break;
                }
            }
        }
    }
    (invalid, invalid2)
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
        assert_eq!(first, 1227775554);
        assert_eq!(second, 4174379265);
    }

    #[test]
    fn test_02() {
        let (first, second) = super::run("input/02.txt");
        assert_eq!(first, 13108371860);
        assert_eq!(second, 22471660255);
    }
}
