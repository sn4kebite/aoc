use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse_paths(filename: &str) -> HashMap<String, HashSet<String>> {
    let mut map = HashMap::new();
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split('-');
        let (from, to) = (split.next().unwrap(), split.next().unwrap());
        let from = String::from(from);
        let to = String::from(to);
        map.entry(from.to_string())
            .or_insert_with(|| HashSet::new())
            .insert(to.to_string());
        map.entry(to).or_insert_with(|| HashSet::new()).insert(from);
    }
    map
}

fn traverse_inner(
    map: &HashMap<String, HashSet<String>>,
    current: &String,
    visited: &HashSet<String>,
    visited_twice: bool,
) -> usize {
    let mut ends = 0;
    let mut visited_twice = visited_twice;
    if visited.contains(current) {
        if visited_twice || current == "start" {
            return ends;
        } else {
            visited_twice = true;
        }
    }
    if current == "end" {
        return ends + 1;
    }
    let mut visited = visited.clone();
    if current.chars().next().unwrap().is_lowercase() {
        visited.insert(current.to_string());
    }
    let path = map.get(current).unwrap();
    for name in path {
        ends += traverse_inner(&map, name, &visited, visited_twice);
    }
    ends
}

fn traverse(map: &HashMap<String, HashSet<String>>, visit_twice: bool) -> usize {
    let visited: HashSet<String> = HashSet::new();
    traverse_inner(&map, &String::from("start"), &visited, !visit_twice)
}

fn run(filename: &str) -> (usize, usize) {
    let map = parse_paths(filename);
    (traverse(&map, false), traverse(&map, true))
}

fn main() {
    println!("{:?}", run("input/12-example1.txt"));
    println!("{:?}", run("input/12-example2.txt"));
    println!("{:?}", run("input/12-example3.txt"));
    println!("{:?}", run("input/12.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example1_12() {
        let (first, second) = super::run("input/12-example1.txt");
        assert_eq!(first, 10);
        assert_eq!(second, 36);
    }

    #[test]
    fn test_example2_12() {
        let (first, second) = super::run("input/12-example2.txt");
        assert_eq!(first, 19);
        assert_eq!(second, 103);
    }

    #[test]
    fn test_example3_12() {
        let (first, second) = super::run("input/12-example3.txt");
        assert_eq!(first, 226);
        assert_eq!(second, 3509);
    }

    #[test]
    fn test_input_12() {
        let (first, second) = super::run("input/12.txt");
        assert_eq!(first, 5254);
        assert_eq!(second, 149385);
    }
}
