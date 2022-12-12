use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_pos(v: &Vec<Vec<u8>>, c: u8) -> (usize, usize) {
    for (y, l) in v.iter().enumerate() {
        if l.contains(&c) {
            return (l.iter().position(|v| *v == c).unwrap(), y);
        }
    }
    (0, 0)
}

fn find_end(map: &Vec<Vec<u8>>, start: &(usize, usize), end: &(usize, usize)) -> usize {
    let width = map[0].len();
    let height = map.len();
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    let mut queue = BinaryHeap::new();
    queue.push(Reverse((0, *start)));
    let mut steps = 0;
    while let Some(Reverse(item)) = queue.pop() {
        let pos = item.1;
        let new_steps = item.0;
        if pos == *end {
            if steps == 0 || new_steps < steps {
                steps = new_steps;
            }
        }
        if visited.get(&pos).unwrap_or(&usize::MAX) <= &new_steps {
            continue;
        }
        visited.insert(pos, new_steps);
        let mut next = vec![];
        if pos.0 > 0 {
            next.push((pos.0 - 1, pos.1));
        }
        if pos.1 > 0 {
            next.push((pos.0, pos.1 - 1));
        }
        if pos.0 < width - 1 {
            next.push((pos.0 + 1, pos.1));
        }
        if pos.1 < height - 1 {
            next.push((pos.0, pos.1 + 1));
        }
        for (x, y) in next {
            let new = map[y][x] as isize;
            let old = map[pos.1][pos.0] as isize;
            if new - old < 2 {
                queue.push(Reverse((new_steps + 1, (x, y))));
            }
        }
    }
    steps
}

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut map: Vec<Vec<u8>> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        map.push(line.bytes().collect());
    }
    let start = find_pos(&map, b'S');
    let end = find_pos(&map, b'E');
    map[start.1][start.0] = b'a';
    map[end.1][end.0] = b'z';
    let steps = find_end(&map, &start, &end);
    let mut steps2 = usize::MAX;
    let width = map[0].len();
    let height = map.len();
    for x in 0..width {
        for y in 0..height {
            if map[y][x] == b'a' {
                let new = find_end(&map, &(x, y), &end);
                // Some starting points does not reach the end and thus returns 0
                if new > 0 {
                    steps2 = steps2.min(new);
                }
            }
        }
    }
    (steps, steps2)
}

fn main() {
    println!("{:?}", run("input/12-example.txt"));
    println!("{:?}", run("input/12.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_12() {
        let (first, second) = super::run("input/12-example.txt");
        assert_eq!(first, 31);
        assert_eq!(second, 29);
    }

    #[test]
    fn test() {
        let (first, second) = super::run("input/12.txt");
        assert_eq!(first, 440);
        assert_eq!(second, 439);
    }
}
