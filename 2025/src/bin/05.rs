use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Clone, Copy, Debug)]
struct Range {
    from: usize,
    to: usize,
}

impl Range {
    pub fn new(from: usize, to: usize) -> Self {
        Self { from, to }
    }

    pub fn len(&self) -> usize {
        self.to - self.from + 1
    }

    pub fn try_merge(&mut self, other: &Self) -> bool {
        let mut merged = false;
        // overlapping from
        if self.from > other.from && self.from <= other.to {
            self.from = other.from;
            if self.to < other.to {
                self.to = other.to;
            }
            merged = true;
        }
        // overlapping to
        if self.to < other.to && self.to >= other.from {
            self.to = other.to;
            if self.from > other.from {
                self.from = other.from;
            }
            merged = true;
        }
        // contained range
        if other.from >= self.from && other.to <= self.to {
            merged = true;
        }
        merged
    }
}

fn merge_ranges(ranges: &mut Vec<Range>) {
    let mut merged = true;
    let mut merged_indices = vec![];
    while merged {
        merged = false;
        for i in 0..ranges.len() {
            if merged_indices.contains(&i) {
                continue;
            }
            for j in 0..ranges.len() {
                if i == j {
                    continue;
                }
                let t = ranges[j];
                if ranges.get_mut(i).unwrap().try_merge(&t) {
                    if !merged_indices.contains(&j) {
                        merged_indices.push(j);
                    }
                    merged = true;
                }
            }
        }
        merged_indices.sort();
        while let Some(i) = merged_indices.pop() {
            ranges.remove(i);
        }
        merged_indices.clear();
    }
}

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut ranges: Vec<Range> = vec![];
    let mut fresh_items = 0;
    let mut fresh_total = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        if let Some((a, b)) = line.split_once('-') {
            let (a, b): (usize, usize) = (a.parse().unwrap(), b.parse().unwrap());
            ranges.push(Range::new(a, b));
        } else {
            let v: usize = line.parse().unwrap();
            for range in &ranges {
                if v >= range.from && v <= range.to {
                    fresh_items += 1;
                    break;
                }
            }
        }
    }
    merge_ranges(&mut ranges);
    for range in ranges {
        fresh_total += range.len();
    }
    (fresh_items, fresh_total)
}

fn main() {
    println!("{:?}", run("input/05-example.txt"));
    println!("{:?}", run("input/05-example2.txt"));
    println!("{:?}", run("input/05.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_05() {
        let (first, second) = super::run("input/05-example.txt");
        assert_eq!(first, 3);
        assert_eq!(second, 14);
    }

    #[test]
    fn test_example2_05() {
        let (first, second) = super::run("input/05-example2.txt");
        assert_eq!(first, 3);
        assert_eq!(second, 14);
    }

    #[test]
    fn test_05() {
        let (first, second) = super::run("input/05.txt");
        assert_eq!(first, 735);
        assert_eq!(second, 344306344403172);
    }
}
