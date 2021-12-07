use std::io;
use std::io::BufRead;

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

fn main() {
    let mut height = 0;
    let mut width = 0;
    let mut map: Vec<bool> = vec![];
    for line in io::stdin().lock().lines() {
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
    let mut product = 1;
    for [xd, yd] in [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]] {
        let trees = map.traverse(xd, yd);
        println!("Trees [{},{}]: {}", xd, yd, trees);
        product *= trees;
    }
    println!("Product: {}", product);
}
