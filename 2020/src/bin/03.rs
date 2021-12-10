use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

struct Map {
    width: usize,
    height: usize,
    map: Vec<bool>,
}

impl Map {
    fn new(width: usize, height: usize, map: Vec<bool>) -> Self {
        Self { width, height, map }
    }

    fn get(&self, x: usize, y: usize) -> bool {
        self.map[y * self.width + (x % self.width)]
    }

    fn traverse(&self, xd: usize, yd: usize) -> usize {
        let mut x = 0;
        let mut y = 0;
        let mut trees = 0;
        while y < self.height {
            if self.get(x, y) {
                trees += 1;
            }
            x += xd;
            y += yd;
        }
        trees
    }
}

fn run(filename: &str) -> (usize, usize) {
    let mut height = 0;
    let mut width = 0;
    let mut map: Vec<bool> = vec![];
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
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
        map.extend(line.chars().map(|c| c == '#'));
    }
    let map = Map::new(width, height, map);
    let mut tree_count = 0;
    let mut product = 1;
    for [xd, yd] in [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]] {
        let trees = map.traverse(xd, yd);
        //println!("Trees [{},{}]: {}", xd, yd, trees);
        if xd == 3 {
            tree_count = trees;
        }
        product *= trees;
    }
    (tree_count, product)
}

fn main() {
    println!("{:?}", run("input/03-example.txt"));
    println!("{:?}", run("input/03.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_03() {
        let (first, second) = super::run("input/03-example.txt");
        assert_eq!(first, 7);
        assert_eq!(second, 336);
    }

    #[test]
    fn test_input_03() {
        let (first, second) = super::run("input/03.txt");
        assert_eq!(first, 278);
        assert_eq!(second, 9709761600);
    }
}
