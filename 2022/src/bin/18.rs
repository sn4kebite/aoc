use std::collections::{HashSet, VecDeque};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_cube(s: &str) -> (isize, isize, isize) {
    let mut split = s.split(',');
    (
        split.next().unwrap().parse().unwrap(),
        split.next().unwrap().parse().unwrap(),
        split.next().unwrap().parse().unwrap(),
    )
}

fn find_origin(
    pos: (isize, isize, isize),
    cubes: &HashSet<(isize, isize, isize)>,
    inside: &mut HashSet<(isize, isize, isize)>
    ) -> bool {
    let mut queue = VecDeque::new();
    queue.push_front(pos);
    let mut visited = HashSet::new();
    while let Some(pos) = queue.pop_front() {
        if pos == (0, 0, 0) || pos == (20, 20, 20) {
            return true;
        }
        if cubes.contains(&pos) || inside.contains(&pos) || visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        queue.push_back((pos.0 - 1, pos.1, pos.2));
        queue.push_back((pos.0, pos.1 - 1, pos.2));
        queue.push_back((pos.0, pos.1, pos.2 - 1));
        queue.push_back((pos.0 + 1, pos.1, pos.2));
        queue.push_back((pos.0, pos.1 + 1, pos.2));
        queue.push_back((pos.0, pos.1, pos.2 + 1));
    }
    inside.extend(visited.iter());
    false
}

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut cubes = HashSet::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let pos = parse_cube(line.as_str());
        cubes.insert(pos);
    }
    let mut area1 = 0;
    let mut area2 = 0;
    let mut inside = HashSet::new();
    for cube in &cubes {
        if !cubes.contains(&(cube.0 + 1, cube.1, cube.2)) {
            area1 += 1;
            if find_origin((cube.0 + 1, cube.1, cube.2), &cubes, &mut inside) {
                area2 += 1;
            }
        }
        if !cubes.contains(&(cube.0 - 1, cube.1, cube.2)) {
            area1 += 1;
            if find_origin((cube.0 - 1, cube.1, cube.2), &cubes, &mut inside) {
                area2 += 1;
            }
        }
        if !cubes.contains(&(cube.0, cube.1 + 1, cube.2)) {
            area1 += 1;
            if find_origin((cube.0, cube.1 + 1, cube.2), &cubes, &mut inside) {
                area2 += 1;
            }
        }
        if !cubes.contains(&(cube.0, cube.1 - 1, cube.2)) {
            area1 += 1;
            if find_origin((cube.0, cube.1 - 1, cube.2), &cubes, &mut inside) {
                area2 += 1;
            }
        }
        if !cubes.contains(&(cube.0, cube.1, cube.2 + 1)) {
            area1 += 1;
            if find_origin((cube.0, cube.1, cube.2 + 1), &cubes, &mut inside) {
                area2 += 1;
            }
        }
        if !cubes.contains(&(cube.0, cube.1, cube.2 - 1)) {
            area1 += 1;
            if find_origin((cube.0, cube.1, cube.2 - 1), &cubes, &mut inside) {
                area2 += 1;
            }
        }
    }
    (area1, area2)
}

fn main() {
    println!("{:?}", run("input/18-example.txt"));
    println!("{:?}", run("input/18.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_18() {
        let (first, second) = super::run("input/18-example.txt");
        assert_eq!(first, 64);
        assert_eq!(second, 58);
    }

    #[test]
    fn test_18() {
        let (first, second) = super::run("input/18.txt");
        assert_eq!(first, 3432);
        assert_eq!(second, 2042);
    }
}
