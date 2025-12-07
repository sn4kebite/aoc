use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn step(manifold: &mut Vec<Vec<char>>, line: usize) -> usize {
    let beams: Vec<bool> = manifold[line - 1]
        .iter()
        .map(|c| *c == 'S' || *c == '|')
        .collect();
    let mut split_indices = HashSet::new();
    for (i, b) in beams.iter().enumerate() {
        if manifold[line][i] != '.' {
            continue;
        }
        if i > 0 && beams[i - 1] && manifold[line][i - 1] == '^' {
            manifold[line][i] = '|';
            split_indices.insert(i - 1);
        } else if i < manifold[line].len() - 1 && beams[i + 1] && manifold[line][i + 1] == '^' {
            manifold[line][i] = '|';
            split_indices.insert(i + 1);
        } else if *b {
            manifold[line][i] = '|';
        }
    }

    split_indices.len()
}

struct Cache {
    cache: HashMap<(usize, usize), usize>,
}

impl Cache {
    pub fn new() -> Self {
        Self {
            cache: HashMap::new(),
        }
    }

    pub fn get(&self, key: (usize, usize)) -> Option<usize> {
        self.cache.get(&key).copied()
    }

    pub fn set(&mut self, key: (usize, usize), value: usize) {
        self.cache.insert(key, value);
    }
}

fn timeline(manifold: &Vec<Vec<char>>, cache: &mut Cache, last_index: usize, line: usize) -> usize {
    if let Some(v) = cache.get((line, last_index)) {
        return v;
    }
    if line >= manifold.len() {
        return 1;
    }
    let value = if manifold[line][last_index] == '^' {
        timeline(&manifold, cache, last_index - 1, line + 1)
            + timeline(&manifold, cache, last_index + 1, line + 1)
    } else {
        timeline(&manifold, cache, last_index, line + 1)
    };
    cache.set((line, last_index), value);
    value
}

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut manifold: Vec<Vec<char>> = reader
        .lines()
        .map(|line| line.unwrap().chars().collect())
        // Skip empty lines
        .filter(|line: &Vec<char>| !line.iter().all(|c| *c == '.'))
        .collect();
    let mut split = 0;
    let start_index = manifold[0].iter().position(|c| *c == 'S').unwrap();
    let timelines = timeline(&manifold, &mut Cache::new(), start_index, 1);
    for line in 1..manifold.len() {
        split += step(&mut manifold, line);
    }
    (split, timelines)
}

fn main() {
    println!("{:?}", run("input/07-example.txt"));
    println!("{:?}", run("input/07.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_07() {
        let (first, second) = super::run("input/07-example.txt");
        assert_eq!(first, 21);
        assert_eq!(second, 40);
    }

    #[test]
    fn test_07() {
        let (first, second) = super::run("input/07.txt");
        assert_eq!(first, 1537);
        assert_eq!(second, 18818811755665);
    }
}
