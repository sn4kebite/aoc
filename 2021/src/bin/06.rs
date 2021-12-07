use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::mem;

fn run_fish(filename: &str, days: usize) -> usize {
    let mut buf = String::new();
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    reader.read_line(&mut buf).expect("Cannot read input");
    let mut fish: Vec<usize> = [0; 9].to_vec();
    buf.trim()
        .split(',')
        .for_each(|s| fish[s.parse::<usize>().unwrap()] += 1_usize);
    for _ in 0..days {
        let new = mem::replace(&mut fish[0], 0);
        for age in 1..fish.len() {
            fish[age - 1] += mem::replace(&mut fish[age], 0);
        }
        fish[6] += new;
        fish[8] += new;
    }
    println!(
        "Number of fish after {} days: {}",
        days,
        fish.iter().sum::<usize>()
    );
    fish.iter().sum()
}

fn run(filename: &str) -> (usize, usize) {
    (run_fish(&filename, 80), run_fish(&filename, 256))
}

fn main() {
    run("input/06-example.txt");
    run("input/06.txt");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_06() {
        let (first, second) = super::run("input/06-example.txt");
        assert_eq!(first, 5934);
        assert_eq!(second, 26984457539);
    }

    #[test]
    fn test_input_06() {
        let (first, second) = super::run("input/06.txt");
        assert_eq!(first, 393019);
        assert_eq!(second, 1757714216975);
    }
}
