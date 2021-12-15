use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Map {
    map: Vec<usize>,
    width: usize,
    height: usize,
}

impl Map {
    fn parse(filename: &str) -> Self {
        let file = File::open(filename).unwrap();
        let reader = BufReader::new(file);
        let mut map = vec![];
        let mut width = 0;
        let mut height = 0;
        for line in reader.lines() {
            let line = line.unwrap();
            if width == 0 {
                width = line.len();
            }
            map.extend(line.chars().map(|c| c.to_digit(10).unwrap() as usize));
            height += 1;
        }
        Self { map, width, height }
    }

    fn find_exit(&self, expanded: bool) -> usize {
        let width = self.width * if expanded { 5 } else { 1 };
        let height = self.height * if expanded { 5 } else { 1 };
        let mut queue = BinaryHeap::new();
        let mut costs = HashMap::new();
        let last = width * height - 1;
        queue.push(Reverse((0, 0)));
        while let Some(Reverse((cost, index))) = queue.pop() {
            let x = index % width;
            let y = index / width;
            let real_index = (y % self.height) * self.width + (x % self.width);
            let this_cost = self.map[real_index];
            let tx = if expanded { x * 5 / width } else { 0 };
            let ty = if expanded { y * 5 / height } else { 0 };
            let this_cost = {
                let v = this_cost + tx + ty;
                if v > 9 {
                    v - 9
                } else {
                    v
                }
            };
            let cost = cost + this_cost;
            if matches!(costs.get(&index), Some(old_cost) if old_cost <= &cost) {
                continue;
            }
            costs.insert(index, cost);
            if x < width - 1 {
                queue.push(Reverse((cost, y * width + x + 1)));
            }
            if y < height - 1 {
                queue.push(Reverse((cost, (y + 1) * width + x)));
            }
            if x > 0 {
                queue.push(Reverse((cost, y * width + x - 1)));
            }
            if y > 0 {
                queue.push(Reverse((cost, (y - 1) * width + x)));
            }
        }
        costs[&last]
    }

    fn find_path(&self, expanded: bool) -> usize {
        self.find_exit(expanded) - self.map[0]
    }
}

fn run(filename: &str) -> (usize, usize) {
    let map = Map::parse(filename);
    (map.find_path(false), map.find_path(true))
}

fn main() {
    println!("{:?}", run("input/15-example.txt"));
    println!("{:?}", run("input/15.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_15() {
        let (first, second) = super::run("input/15-example.txt");
        assert_eq!(first, 40);
        assert_eq!(second, 315);
    }

    #[test]
    fn test_input_15() {
        let (first, second) = super::run("input/15.txt");
        assert_eq!(first, 553);
        assert_eq!(second, 2858);
    }
}
