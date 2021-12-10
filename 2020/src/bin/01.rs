use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let expenses: Vec<usize> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            match line.parse() {
                Ok(num) => num,
                Err(_) => panic!("Failed to parse {}", line),
            }
        })
        .collect();
    let mut expenses1 = 0;
    let mut expenses2 = 0;
    for i in 0..expenses.len() {
        let a = expenses[i];
        for j in i + 1..expenses.len() {
            let b = expenses[j];
            if a + b == 2020 {
                expenses1 = a * b;
            }
            for k in j + 1..expenses.len() {
                let c = expenses[k];
                if a + b + c == 2020 {
                    expenses2 = a * b * c;
                }
            }
        }
    }
    (expenses1, expenses2)
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
        assert_eq!(first, 514579);
        assert_eq!(second, 241861950);
    }

    #[test]
    fn test_input_01() {
        let (first, second) = super::run("input/01.txt");
        assert_eq!(first, 989824);
        assert_eq!(second, 66432240);
    }
}
