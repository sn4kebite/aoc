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

fn run(filename: &str) -> (usize, usize) {
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
    (first_fold, 0)
}

fn main() {
    println!("{:?}", run("input/13-example.txt"));
    println!("{:?}", run("input/13.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_13() {
        let (first, _) = super::run("input/13-example.txt");
        assert_eq!(first, 17);
    }

    #[test]
    fn test_input_13() {
        let (first, _) = super::run("input/13.txt");
        assert_eq!(first, 729);
    }
}
