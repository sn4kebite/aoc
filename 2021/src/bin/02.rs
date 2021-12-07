use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn run(filename: &str) -> (i32, i32) {
    let mut position = 0;
    let mut depth = 0;
    let mut aim = 0;
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();
        let v: Vec<&str> = line.split_whitespace().collect();
        let dir = v[0];
        let units: i32 = v[1].parse().unwrap();
        match dir {
            "forward" => {
                position += units;
                depth += aim * units;
            }
            "up" => aim -= units,
            "down" => aim += units,
            _ => panic!("Invalid direction '{}'", dir),
        }
    }
    println!("Position: {}", position);
    println!("Old depth: {}", aim);
    println!("Real depth: {}", depth);
    println!("Old result: {}", position * aim);
    println!("Real result: {}", position * depth);
    (position * aim, position * depth)
}

fn main() {
    run("input/02-example.txt");
    run("input/02.txt");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_02() {
        let (first, second) = super::run("input/02-example.txt");
        assert_eq!(first, 150);
        assert_eq!(second, 900);
    }

    #[test]
    fn test_input_02() {
        let (first, second) = super::run("input/02.txt");
        assert_eq!(first, 1813801);
        assert_eq!(second, 1960569556);
    }
}
