use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut groups = vec![];
    let mut group = HashMap::new();
    let mut group_sizes = vec![0];
    for line in reader.lines() {
        let line = line.unwrap();
        if line.len() == 0 {
            groups.push(group);
            group = HashMap::new();
            group_sizes.push(0);
            continue;
        }
        for c in line.chars() {
            *group.entry(c).or_insert(0) += 1;
        }
        *group_sizes.last_mut().unwrap() += 1;
    }
    groups.push(group);
    (
        groups.iter().map(|group| group.len()).sum(),
        groups
            .iter()
            .enumerate()
            .map(|(i, group)| {
                group
                    .values()
                    .map(|a| if *a == group_sizes[i] { 1 } else { 0 })
                    .sum::<usize>()
            })
            .sum(),
    )
}

fn main() {
    println!("{:?}", run("input/06-example.txt"));
    println!("{:?}", run("input/06.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_06() {
        let (first, second) = super::run("input/06-example.txt");
        assert_eq!(first, 11);
        assert_eq!(second, 6);
    }

    #[test]
    fn test_input_06() {
        let (first, second) = super::run("input/06.txt");
        assert_eq!(first, 6742);
        assert_eq!(second, 3447);
    }
}
