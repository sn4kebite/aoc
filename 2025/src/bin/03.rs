use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_max(digits: &Vec<u8>, start: usize, levels: u32) -> usize {
    if levels == 0 {
        return 0;
    }
    let mut tot = 0;
    let mut max = 0;
    for i in start..digits.len() - levels as usize + 1 {
        let v = digits[i] as usize;
        if v > max {
            let tv = (10_usize).pow(levels - 1) * v as usize + find_max(&digits, i + 1, levels - 1);
            if tv > tot {
                max = v;
                tot = tv;
            }
        }
    }
    tot
}

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut sum2 = 0;
    let mut sum12 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let digits: Vec<u8> = line.bytes().map(|c| c - b'0').collect();
        sum2 += find_max(&digits, 0, 2);
        sum12 += find_max(&digits, 0, 12);
    }
    (sum2, sum12)
}

fn main() {
    println!("{:?}", run("input/03-example.txt"));
    println!("{:?}", run("input/03.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_03() {
        let (first, second) = super::run("input/03-example.txt");
        assert_eq!(first, 357);
        assert_eq!(second, 3121910778619);
    }

    #[test]
    fn test_03() {
        let (first, second) = super::run("input/03.txt");
        assert_eq!(first, 17229);
        assert_eq!(second, 170520923035051);
    }
}
