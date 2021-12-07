use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Clone, Debug)]
struct BitCount {
    ones: usize,
    zeroes: usize,
}

fn analyze_report(v: &Vec<usize>, bits: usize) -> Vec<BitCount> {
    let mut ret: Vec<BitCount> = vec![];
    for _ in 0..bits {
        ret.push(BitCount { ones: 0, zeroes: 0 });
    }
    for n in v.iter() {
        for i in 0..bits {
            if n & (1 << i) > 0 {
                ret[i].ones += 1;
            } else {
                ret[i].zeroes += 1;
            }
        }
    }
    ret
}

fn extract_ratings(v: &Vec<usize>, bits: usize, positive: bool) -> usize {
    let mut pos = bits - 1;
    let mut temp = v.clone();
    let mut counts = analyze_report(&v, bits);
    while temp.len() > 1 {
        let mut new_v: Vec<usize> = vec![];
        for n in &temp {
            let n = *n;
            let is_one = n & (1 << pos) > 0;
            let is_pos = counts[pos].ones >= counts[pos].zeroes;
            if (positive && is_one == is_pos) || (!positive && is_one != is_pos) {
                new_v.push(n);
            }
        }
        pos = if pos == 0 { bits } else { pos } - 1;
        temp = new_v;
        counts = analyze_report(&temp, bits);
    }
    temp[0]
}

fn run(filename: &str) -> (usize, usize) {
    let mut v: Vec<usize> = vec![];
    let mut bits = 0;
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();
        v.push(usize::from_str_radix(line, 2).unwrap());
        if bits == 0 {
            bits = line.len();
        }
    }
    let counts = analyze_report(&v, bits);
    let mut gamma = 0;
    let mut epsilon = 0;
    for (i, count) in counts.iter().enumerate() {
        if count.ones > count.zeroes {
            gamma += 1 << i;
        } else {
            epsilon += 1 << i;
        }
    }
    //println!("gamma={} epsilon={}", gamma, epsilon);
    println!("Power consumption: {}", gamma * epsilon);
    let o2 = extract_ratings(&v, bits, true);
    let co2 = extract_ratings(&v, bits, false);
    println!("O₂ generator rating: {}", o2);
    println!("CO₂ scrubber rating: {}", co2);
    println!("Life support rating: {}", o2 * co2);
    (gamma * epsilon, o2 * co2)
}

fn main() {
    run("input/03-example.txt");
    run("input/03.txt");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_03() {
        let (first, second) = super::run("input/03-example.txt");
        assert_eq!(first, 198);
        assert_eq!(second, 230);
    }

    #[test]
    fn test_input_03() {
        let (first, second) = super::run("input/03.txt");
        assert_eq!(first, 2967914);
        assert_eq!(second, 7041258);
    }
}
