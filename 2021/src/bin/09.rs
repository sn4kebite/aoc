use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn find_basin_size(
    map: &Vec<usize>,
    start_x: usize,
    start_y: usize,
    width: usize,
    height: usize,
) -> usize {
    let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
    let mut visited: HashSet<(usize, usize)> = HashSet::new();
    queue.push_back((start_x, start_y));
    while let Some((x, y)) = queue.pop_front() {
        if map[y * width + x] == 9 || visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        if x < width - 1 {
            queue.push_back((x + 1, y));
        }
        if x > 0 {
            queue.push_back((x - 1, y));
        }
        if y < height - 1 {
            queue.push_back((x, y + 1));
        }
        if y > 0 {
            queue.push_back((x, y - 1));
        }
    }
    visited
        .iter()
        .filter(|&&(x, y)| map[y * width + x] < 9)
        .count()
}

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut width = 0;
    let mut height = 0;
    let mut map: Vec<usize> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.len() == 0 {
            continue;
        }
        if width == 0 {
            width = line.len();
        }
        height += 1;
        map.extend(line.bytes().map(|c| (c - b'0') as usize));
    }
    let mut low_count = 0;
    let mut low_sum = 0;
    let mut basins: Vec<usize> = vec![];
    for x in 0..width {
        for y in 0..height {
            let current = map[y * width + x];
            let mut low = true;
            if x > 0 && current >= map[y * width + x - 1] {
                low = false;
            }
            if x < width - 1 && current >= map[y * width + x + 1] {
                low = false;
            }
            if y > 0 && current >= map[(y - 1) * width + x] {
                low = false;
            }
            if y < height - 1 && current >= map[(y + 1) * width + x] {
                low = false;
            }
            if low {
                low_sum += current;
                low_count += 1;
                basins.push(find_basin_size(&map, x, y, width, height));
            }
        }
    }
    basins.sort();
    (
        low_count + low_sum,
        basins[basins.len() - 3..].iter().product(),
    )
}

fn main() {
    println!("{:?}", run("input/09-example.txt"));
    println!("{:?}", run("input/09.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_09() {
        let (first, second) = super::run("input/09-example.txt");
        assert_eq!(first, 15);
        assert_eq!(second, 1134);
    }

    #[test]
    fn test_input_09() {
        let (first, second) = super::run("input/09.txt");
        assert_eq!(first, 498);
        assert_eq!(second, 1071000);
    }
}
