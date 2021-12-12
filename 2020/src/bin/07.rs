use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut bag_map = HashMap::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split(" contain ");
        let (bag, bags) = (split.next().unwrap(), split.next().unwrap());
        // strip "bags" suffix
        let (bag, _) = bag.rsplit_once(' ').unwrap();
        let bags: HashMap<String, usize> = bags
            .split(", ")
            .filter_map(|b| {
                if b == "no other bags." {
                    return None;
                }
                let (num, name) = b.split_once(' ').unwrap();
                // strip suffix
                let (name, _) = name.rsplit_once(' ').unwrap();
                Some((name.to_string(), num.parse().unwrap()))
            })
            .collect();
        bag_map.insert(bag.to_string(), bags);
    }
    let mut queue = VecDeque::from(["shiny gold"]);
    let mut valid_bags = HashSet::new();
    while let Some(bag) = queue.pop_front() {
        for (b, m) in bag_map.iter() {
            if m.contains_key(bag) {
                valid_bags.insert(b);
                queue.push_back(b);
            }
        }
    }
    let mut total_bags = 0;
    let mut queue = VecDeque::from([("shiny gold", 1)]);
    while let Some((bag, count)) = queue.pop_front() {
        if let Some(m) = bag_map.get(bag) {
            for (name, c) in m.iter() {
                total_bags += count * c;
                queue.push_back((name, count * c));
            }
        }
    }
    (valid_bags.len(), total_bags)
}

fn main() {
    println!("{:?}", run("input/07-example1.txt"));
    println!("{:?}", run("input/07-example2.txt"));
    println!("{:?}", run("input/07.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example1_07() {
        let (first, second) = super::run("input/07-example1.txt");
        assert_eq!(first, 4);
        assert_eq!(second, 32);
    }

    #[test]
    fn test_example2_07() {
        let (_, second) = super::run("input/07-example2.txt");
        assert_eq!(second, 126);
    }

    #[test]
    fn test_input_07() {
        let (first, second) = super::run("input/07.txt");
        assert_eq!(first, 119);
        assert_eq!(second, 155802);
    }
}
