use std::collections::HashSet;
use std::collections::VecDeque;
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
            let line = line.trim();
            if width == 0 {
                width = line.len();
            }
            map.extend(line.chars().map(|c| c.to_digit(10).unwrap() as usize));
            height += 1;
        }
        Self { map, width, height }
    }

    pub fn get_size(&self) -> usize {
        return self.width * self.height;
    }

    fn get_neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut tiles = vec![];
        if x > 0 {
            tiles.push((x - 1, y));
            if y > 0 {
                tiles.push((x - 1, y - 1));
            }
            if y < self.height - 1 {
                tiles.push((x - 1, y + 1));
            }
        }
        if x < self.width - 1 {
            tiles.push((x + 1, y));
            if y > 0 {
                tiles.push((x + 1, y - 1));
            }
            if y < self.height - 1 {
                tiles.push((x + 1, y + 1));
            }
        }
        if y > 0 {
            tiles.push((x, y - 1));
        }
        if y < self.height - 1 {
            tiles.push((x, y + 1));
        }
        tiles
    }

    fn step(&mut self) -> usize {
        let mut queue: VecDeque<(usize, usize)> = VecDeque::new();
        for (i, v) in self.map.iter_mut().enumerate() {
            *v += 1;
            if *v > 9 {
                queue.push_back((i % self.width, i / self.width));
            }
        }
        let mut flashed = HashSet::new();
        while let Some((x, y)) = queue.pop_front() {
            if flashed.contains(&(x, y)) {
                continue;
            }
            flashed.insert((x, y));
            for (nx, ny) in self.get_neighbors(x, y) {
                let o = self.map.get_mut(ny * self.width + nx).unwrap();
                *o += 1;
                if *o > 9 {
                    queue.push_back((nx, ny));
                }
            }
        }
        for (x, y) in &flashed {
            self.map[y * self.width + x] = 0;
        }
        flashed.len()
    }
}

fn run(filename: &str) -> (usize, usize) {
    let mut map = Map::parse(filename);
    let size = map.get_size();
    let mut flashes = 0;
    let mut sync_step = 0;
    for step in 1.. {
        let f = map.step();
        if f == size && sync_step == 0 {
            sync_step = step;
        }
        if step <= 100 {
            flashes += f;
        }
        if step >= 100 && sync_step > 0 {
            break;
        }
    }
    (flashes, sync_step)
}

fn main() {
    println!("{:?}", run("input/11-example.txt"));
    println!("{:?}", run("input/11.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_11() {
        let (first, second) = super::run("input/11-example.txt");
        assert_eq!(first, 1656);
        assert_eq!(second, 195);
    }

    #[test]
    fn test_input_11() {
        let (first, second) = super::run("input/11.txt");
        assert_eq!(first, 1675);
        assert_eq!(second, 515);
    }
}
