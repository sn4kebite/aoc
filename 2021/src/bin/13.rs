use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn fold(map: &HashSet<(usize, usize)>, fx: usize, fy: usize) -> HashSet<(usize, usize)> {
    let mut new = HashSet::new();
    for dot in map {
        let (mut x, mut y) = dot;
        if fx > 0 && x > fx {
            x -= (x - fx) * 2
        }
        if fy > 0 && y > fy {
            y -= (y - fy) * 2
        }
        new.insert((x, y));
    }
    new
}

fn print_map(map: &HashSet<(usize, usize)>) {
    let mut width = 0;
    let mut height = 0;
    for (x, y) in map {
        if *x > width {
            width = *x;
        }
        if *y > height {
            height = *y;
        }
    }
    for y in 0..height + 1 {
        for x in 0..width + 1 {
            print!(
                "{}",
                match map.get(&(x, y)) {
                    Some(_) => '#',
                    None => ' ',
                }
            );
        }
        println!();
    }
}

fn run(filename: &str) -> (usize, HashSet<(usize, usize)>) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut map = HashSet::new();
    let mut buf = String::new();
    while let Ok(_) = &reader.read_line(&mut buf) {
        let line = buf.trim();
        if line.len() == 0 {
            break;
        }
        let (a, b) = {
            let mut split = line.split(',');
            (
                split.next().unwrap().parse::<usize>().unwrap(),
                split.next().unwrap().parse::<usize>().unwrap(),
            )
        };
        map.insert((a, b));
        buf.clear();
    }
    let mut first_fold = 0;
    while let Ok(_) = reader.read_line(&mut buf) {
        let line = buf.trim();
        if line.len() == 0 {
            break;
        }
        let mut split = line.split_whitespace();
        split.next().unwrap();
        split.next().unwrap();
        let (what, value) = split.next().unwrap().split_once('=').unwrap();
        let value: usize = value.parse().unwrap();
        match what {
            "x" => map = fold(&map, value, 0),
            "y" => map = fold(&map, 0, value),
            _ => panic!("invalid fold"),
        }
        if first_fold == 0 {
            first_fold = map.len();
        }
        buf.clear();
    }
    print_map(&map);
    (first_fold, map)
}

fn main() {
    println!("{:?}", run("input/13-example.txt"));
    println!("{:?}", run("input/13.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_13() {
        let (first, second) = super::run("input/13-example.txt");
        assert_eq!(first, 17);
        assert_eq!(
            second,
            super::HashSet::from([
                (1, 4),
                (3, 0),
                (4, 3),
                (0, 0),
                (0, 1),
                (0, 2),
                (3, 4),
                (1, 0),
                (0, 4),
                (4, 0),
                (4, 4),
                (4, 2),
                (2, 4),
                (0, 3),
                (4, 1),
                (2, 0),
            ])
        );
    }

    #[test]
    fn test_input_13() {
        let (first, second) = super::run("input/13.txt");
        assert_eq!(first, 729);
        assert_eq!(
            second,
            super::HashSet::from([
                (20, 4),
                (6, 0),
                (20, 5),
                (32, 0),
                (2, 0),
                (7, 0),
                (2, 4),
                (23, 1),
                (35, 4),
                (23, 4),
                (30, 3),
                (7, 5),
                (28, 2),
                (25, 1),
                (25, 4),
                (0, 2),
                (0, 1),
                (35, 0),
                (7, 3),
                (15, 4),
                (21, 2),
                (2, 3),
                (37, 0),
                (30, 4),
                (25, 5),
                (22, 5),
                (28, 5),
                (3, 5),
                (30, 5),
                (13, 5),
                (36, 3),
                (10, 0),
                (12, 5),
                (27, 2),
                (30, 2),
                (21, 5),
                (5, 2),
                (5, 3),
                (10, 5),
                (8, 3),
                (11, 0),
                (12, 2),
                (0, 0),
                (20, 3),
                (25, 3),
                (6, 5),
                (5, 1),
                (13, 0),
                (5, 4),
                (20, 2),
                (10, 4),
                (35, 3),
                (31, 0),
                (37, 3),
                (35, 1),
                (22, 2),
                (21, 0),
                (28, 1),
                (35, 5),
                (11, 3),
                (16, 5),
                (33, 0),
                (32, 2),
                (8, 1),
                (15, 5),
                (15, 3),
                (18, 5),
                (22, 0),
                (15, 0),
                (12, 0),
                (0, 3),
                (30, 0),
                (15, 2),
                (28, 0),
                (1, 0),
                (0, 4),
                (0, 5),
                (8, 5),
                (28, 3),
                (1, 3),
                (20, 0),
                (13, 1),
                (17, 5),
                (25, 0),
                (30, 1),
                (20, 1),
                (3, 1),
                (38, 2),
                (36, 0),
                (15, 1),
                (8, 4),
                (25, 2),
                (3, 2),
                (28, 4),
                (23, 3),
                (31, 2),
                (35, 2),
                (26, 2),
                (38, 1),
                (11, 5),
            ])
        );
    }
}
