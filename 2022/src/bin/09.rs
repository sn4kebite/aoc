use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn _print_rope(knots: &[(isize, isize); 10]) {
    let minx = *knots.map(|x| x.0).iter().min().unwrap();
    let maxx = *knots.map(|x| x.0).iter().max().unwrap();
    let miny = *knots.map(|x| x.1).iter().min().unwrap();
    let maxy = *knots.map(|x| x.1).iter().max().unwrap();
    for y in (miny..maxy + 1).rev() {
        for x in minx..maxx + 1 {
            let mut found_knot = false;
            for (i, k) in knots.iter().enumerate() {
                if k.0 == x && k.1 == y {
                    if i == 0 {
                        print!("H");
                    } else {
                        print!("{}", i);
                    }
                    found_knot = true;
                    break;
                }
            }
            if !found_knot {
                print!(".");
            }
        }
        println!();
    }
}

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut visited_first = HashSet::new();
    let mut visited_last = HashSet::new();
    let mut knots = [(0isize, 0isize); 10];
    visited_first.insert(knots[1]);
    visited_last.insert(knots[9]);
    for line in reader.lines() {
        let line = line.unwrap();
        let (dir, steps) = line.split_once(' ').unwrap();
        let dir = match dir {
            "U" => (0, 1),
            "D" => (0, -1),
            "R" => (1, 0),
            "L" => (-1, 0),
            _ => panic!("unknown dir {}", dir),
        };
        let steps = steps.parse::<usize>().unwrap();
        for _ in 0..steps {
            let mut head = &mut knots[0];
            head.0 += dir.0;
            head.1 += dir.1;
            let mut next = head.clone();
            for knot in knots[1..].iter_mut() {
                if (next.0 - knot.0).abs() > 1 || (next.1 - knot.1).abs() > 1 {
                    let fix_dir = (
                        (next.0 - knot.0).clamp(-1, 1),
                        (next.1 - knot.1).clamp(-1, 1),
                    );
                    knot.0 += fix_dir.0;
                    knot.1 += fix_dir.1;
                }
                next = knot.clone();
            }
            visited_first.insert(knots[1]);
            visited_last.insert(knots[9]);
        }
        //print_rope(&knots);
    }
    (visited_first.len(), visited_last.len())
}

fn main() {
    println!("{:?}", run("input/09-example1.txt"));
    println!("{:?}", run("input/09-example2.txt"));
    println!("{:?}", run("input/09.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example1_09() {
        let (first, second) = super::run("input/09-example1.txt");
        assert_eq!(first, 13);
        assert_eq!(second, 1);
    }

    #[test]
    fn test_example2_09() {
        let (_, second) = super::run("input/09-example2.txt");
        assert_eq!(second, 36);
    }

    #[test]
    fn test_09() {
        let (first, second) = super::run("input/09.txt");
        assert_eq!(first, 6256);
        assert_eq!(second, 2665);
    }
}
