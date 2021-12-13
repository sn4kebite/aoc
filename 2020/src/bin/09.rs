use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn verify(preamble: &[usize], n: usize) -> bool {
    for i in 0..preamble.len() - 1 {
        for j in i + 1..preamble.len() {
            if preamble[i] + preamble[j] == n {
                return true;
            }
        }
    }
    false
}

fn find_weakness(numbers: &Vec<usize>, n: usize) -> usize {
    for i in 0..numbers.len() - 1 {
        let mut lo = numbers[i];
        let mut hi = lo;
        let mut sum = lo;
        for j in i + 1..numbers.len() {
            let value = numbers[j];
            if value < lo {
                lo = value;
            }
            if value > hi {
                hi = value;
            }
            sum += value;
            if sum == n {
                return lo + hi;
            }
            if sum > n {
                break;
            }
        }
    }
    0
}

fn run(filename: &str, preamble_size: usize) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut numbers = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let n: usize = line.parse().unwrap();
        if numbers.len() >= preamble_size && !verify(&numbers[numbers.len() - preamble_size..], n) {
            numbers.push(n);
            return (n, find_weakness(&numbers, n));
        }
        numbers.push(n);
    }
    (0, 0)
}

fn main() {
    println!("{:?}", run("input/09-example.txt", 5));
    println!("{:?}", run("input/09.txt", 25));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_09() {
        let (first, second) = super::run("input/09-example.txt", 5);
        assert_eq!(first, 127);
        assert_eq!(second, 62);
    }

    #[test]
    fn test_input_09() {
        let (first, second) = super::run("input/09.txt", 25);
        assert_eq!(first, 530627549);
        assert_eq!(second, 77730285);
    }
}
