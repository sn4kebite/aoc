use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn u8_to_pri(v: u8) -> u8 {
    if v <= b'Z' {
        v - b'A' + 27
    } else {
        v - b'a' + 1
    }
}

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut prio_sum = 0;
    let mut group_sum = 0;
    let mut group = 0;
    let mut group_set = HashSet::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let l = line.len() / 2;
        let h1: HashSet<_> = line[0..l].bytes().collect();
        let h2: HashSet<_> = line[l..line.len()].bytes().collect();
        let inter = h1.intersection(&h2);
        prio_sum += inter.map(|i| u8_to_pri(*i) as usize).sum::<usize>();
        let set: HashSet<_> = line.bytes().collect();
        if group == 0 {
            group_set.extend(set);
        } else {
            group_set = group_set.intersection(&set).map(|v| *v).collect();
        }
        group += 1;
        if group == 3 {
            if group_set.len() > 0 {
                group_sum += u8_to_pri(*group_set.iter().next().unwrap()) as usize;
            }
            group_set.clear();
            group = 0;
        }
    }
    (prio_sum, group_sum)
}

fn main() {
    println!("{:?}", run("input/03-example.txt"));
    println!("{:?}", run("input/03.txt"));
}

#[cfg(test)]
mod tests {
    use super::u8_to_pri;

    #[test]
    fn test_prio() {
        assert_eq!(u8_to_pri(b'a'), 1);
        assert_eq!(u8_to_pri(b'z'), 26);
        assert_eq!(u8_to_pri(b'A'), 27);
        assert_eq!(u8_to_pri(b'Z'), 52);
    }

    #[test]
    fn test_example_03() {
        let (first, second) = super::run("input/03-example.txt");
        assert_eq!(first, 157);
        assert_eq!(second, 70);
    }

    #[test]
    fn test_03() {
        let (first, second) = super::run("input/03.txt");
        assert_eq!(first, 8053);
        assert_eq!(second, 2425);
    }
}
