use std::fs::File;
use std::io::{BufRead, BufReader};

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut number: isize = 50;
    let mut count = 0;
    let mut passes: usize = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let right = line.chars().nth(0).unwrap() == 'R';
        let amt: usize = line[1..].parse().unwrap();
        if right {
            number = number + amt as isize;
            passes += number as usize / 100;
            number %= 100;
        } else {
            let was_zero = number == 0;
            number = number - amt as isize;
            if number <= 0 {
                passes += number.abs() as usize / 100;
                if !was_zero {
                    passes += 1;
                }
                number = (number + number.abs() / 100 * 100 + 100) % 100;
            } else {
                passes += number as usize / 100;
                number %= 100;
            }
        }
        if number == 0 {
            count += 1;
        }
    }
    (count, passes)
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
        assert_eq!(first, 3);
        assert_eq!(second, 6);
    }

    #[test]
    fn test_01() {
        let (first, second) = super::run("input/01.txt");
        assert_eq!(first, 980);
        assert_eq!(second, 5961);
    }
}
