use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn run_(filename: &str, floor: bool) -> usize {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut map = HashSet::new();
    let mut floor_y = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let mut previous: Option<(usize, usize)> = None;
        for p in line.split(" -> ") {
            let p = p.split_once(',');
            let p: (usize, usize) = (p.unwrap().0.parse().unwrap(), p.unwrap().1.parse().unwrap());
            if let Some(prev) = previous {
                let minx = prev.0.min(p.0);
                let maxx = prev.0.max(p.0) + 1;
                let miny = prev.1.min(p.1);
                let maxy = prev.1.max(p.1) + 1;
                floor_y = floor_y.max(maxy);
                for x in minx..maxx {
                    for y in miny..maxy {
                        map.insert((x, y));
                    }
                }
            }
            previous = Some(p);
        }
    }
    floor_y += 1; // +1 only since we already add 1 to maxy above
    let sand_origin = (500, 0);
    let mut sand = 0;
    'it: loop {
        let mut current = sand_origin;
        for i in 0.. {
            if i > 1000 {
                break 'it;
            }
            if floor && current.1 + 1 == floor_y {
                if !map.insert(current) {
                    panic!("tried to insert an already existing position");
                }
                sand += 1;
                break;
            }
            let next = (current.0, current.1 + 1);
            if !map.contains(&next) {
                current = next;
                continue;
            }
            let next = (current.0 - 1, current.1 + 1);
            if !map.contains(&next) {
                current = next;
                continue;
            }
            let next = (current.0 + 1, current.1 + 1);
            if !map.contains(&next) {
                current = next;
                continue;
            }
            if !map.insert(current) {
                panic!("tried to insert an already existing position");
            }
            sand += 1;
            break;
        }
        if floor && map.contains(&sand_origin) {
            break;
        }
    }
    sand
}

fn run(filename: &str) -> (usize, usize) {
    (run_(filename, false), run_(filename, true))
}

fn main() {
    println!("{:?}", run("input/14-example.txt"));
    println!("{:?}", run("input/14.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_14() {
        let (first, second) = super::run("input/14-example.txt");
        assert_eq!(first, 24);
        assert_eq!(second, 93);
    }

    #[test]
    fn test_14() {
        let (first, second) = super::run("input/14.txt");
        assert_eq!(first, 1078);
        assert_eq!(second, 30157);
    }
}
