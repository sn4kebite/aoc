use std::collections::VecDeque;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn run(filename: &str) -> (i32, i32) {
    let mut increased = 0;
    let mut window: VecDeque<i32> = VecDeque::new();
    let mut last_sum: Option<i32> = None;
    let mut sum_increased = 0;
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let value: i32 = match line.parse() {
            Ok(num) => num,
            Err(_) => panic!("Failed to parse {}", line),
        };
        window.push_front(value);
        if window.len() > 1 {
            if value > window[1] {
                increased += 1
            }
        }
        window.truncate(3);
        if window.len() < 3 {
            continue;
        }
        let sum: i32 = window.iter().sum();
        match last_sum {
            Some(sv) => {
                if sum > sv {
                    sum_increased += 1
                }
            }
            _ => (),
        }
        last_sum = Some(sum);
    }
    println!("Increased {} times", increased);
    println!("Sum increased {} times", sum_increased);
    (increased, sum_increased)
}

fn main() {
    run("input/01.txt");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_01() {
        let (first, second) = super::run("input/01-example.txt");
        assert_eq!(first, 7);
        assert_eq!(second, 5);
    }

    #[test]
    fn test_input_01() {
        let (first, second) = super::run("input/01.txt");
        assert_eq!(first, 1711);
        assert_eq!(second, 1743);
    }
}
