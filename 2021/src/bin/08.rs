use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn sorted_str(s: &str) -> String {
    let mut v: Vec<char> = s.chars().collect();
    v.sort();
    v.iter().cloned().collect::<String>()
}

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut sum1 = 0;
    let mut sum2 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.trim().split(" | ");
        let (patterns, output) = (split.next().unwrap(), split.next().unwrap());
        let patterns: Vec<String> = patterns.split_whitespace().map(|s| sorted_str(s)).collect();
        let output: Vec<String> = output.split_whitespace().map(|s| sorted_str(s)).collect();
        //println!("{:?} {:?}", patterns, output);
        let mut numbers: Vec<&str> = vec![];
        numbers.resize(10, "");
        sum1 += output
            .iter()
            .map(|s| {
                if s.len() == 2 || s.len() == 3 || s.len() == 4 || s.len() == 7 {
                    1
                } else {
                    0
                }
            })
            .sum::<usize>();
        for p in &patterns {
            if p.len() == 2 {
                numbers[1] = p;
            }
            if p.len() == 3 {
                numbers[7] = p;
            }
            if p.len() == 4 {
                numbers[4] = p;
            }
            if p.len() == 7 {
                numbers[8] = p;
            }
        }
        for p in &patterns {
            if p.len() == 6 && numbers[4].chars().all(|c| p.contains(c)) {
                numbers[9] = p;
            }
            if p.len() == 6
                && numbers[1]
                    .chars()
                    .map(|c| p.contains(c) as usize)
                    .sum::<usize>()
                    == 1
            {
                numbers[6] = p;
            }
            if p.len() == 5 && numbers[7].chars().all(|c| p.contains(c)) {
                numbers[3] = p;
            }
        }
        for p in &patterns {
            if p.len() == 5
                && numbers[7]
                    .chars()
                    .map(|c| p.contains(c) as usize)
                    .sum::<usize>()
                    == 2
                && numbers[6]
                    .chars()
                    .map(|c| p.contains(c) as usize)
                    .sum::<usize>()
                    == 5
            {
                numbers[5] = p;
                break;
            }
        }
        for p in &patterns {
            if p.len() == 5 && !numbers.contains(&p.as_str()) {
                numbers[2] = p;
            }
            if p.len() == 6 && !numbers.contains(&p.as_str()) {
                numbers[0] = p;
            }
        }
        if numbers.iter().any(|s| s.len() == 0) {
            panic!("missing numbers in {:?}", numbers);
        }
        let mut digits = 0;
        for s in output {
            match numbers.iter().position(|x| *x == s) {
                Some(index) => digits = digits * 10 + index,
                None => panic!("missing number for {}", s),
            };
        }
        sum2 += digits;
        //println!("numbers={:?}", numbers);
    }
    (sum1, sum2)
}

fn main() {
    println!("{:?}", run("input/08-example.txt"));
    println!("{:?}", run("input/08.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_08() {
        let (first, second) = super::run("input/08-example.txt");
        assert_eq!(first, 26);
        assert_eq!(second, 61229);
    }

    #[test]
    fn test_input_08() {
        let (first, second) = super::run("input/08.txt");
        assert_eq!(first, 412);
        assert_eq!(second, 978171);
    }
}
