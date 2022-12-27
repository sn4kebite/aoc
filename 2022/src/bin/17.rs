use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

const SHAPES: [[bool; 16]; 5] = [
    [false, false, false, false,
    false, false, false, false,
    false, false, false, false,
    true, true, true, true],
    [false, false, false, false,
    false, true, false, false,
    true, true, true, false,
    false, true, false, false],
    [false, false, false, false,
    false, false, true, false,
    false, false, true, false,
    true, true, true, false],
    [true, false, false, false,
    true, false, false, false,
    true, false, false, false,
    true, false, false, false],
    [false, false, false, false,
    false, false, false, false,
    true, true, false, false,
    true, true, false, false]
];

fn _print_rocks(rocks: &HashSet<(usize, usize)>) {
    let max_y = rocks.iter().map(|r| r.1).max().unwrap();
    println!("|.......|");
    for y in (0..max_y+1).rev() {
        print!("|");
        for x in 0..7 {
            print!("{}", if rocks.contains(&(x, y)) { '#' } else { '.' });
        }
        println!("|");
    }
    println!("+-------+");
}

fn test_shape(
    shape: &[bool; 16],
    rocks: &HashSet<(usize, usize)>,
    pos: (usize, usize),
    offset: (isize, isize)) -> bool {
    for y in 0..4 {
        for x in 0..4 {
            let i = (3 - y) * 4 + x;
            let x = pos.0 as isize + x as isize + offset.0;
            let y = pos.1 as isize + y as isize + offset.1;
            let k = (x as usize, y as usize);
            if shape[i] && (x < 0 || x > 6 || rocks.contains(&k)) {
                return true;
            }
        }
    }
    false
}

fn find_pattern<T>(v: &Vec<T>, min_size: usize, max_size: usize) -> Option<(usize, usize)>
    where T: PartialEq
{
    if v.len() < min_size * 2 {
        return None;
    }
    for size in min_size..max_size+1 {
        let match_part = &v[v.len() - size..];
        if v[v.len() - size * 2..v.len() - size] == *match_part {
            return Some((v.len() - size, v.len()));
        }
    }
    None
}

fn run(filename: &str, min_pattern_size: usize, max_pattern_size: usize) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut line = String::new();
    reader.read_line(&mut line).expect("missing first line");
    let line = line.trim();
    let jet_patterns: Vec<isize> = line.chars().map(|c| match c {
            '<' => -1,
            '>' => 1,
            _ => panic!("Unknown pattern direction {}", c),
        }).collect();
    let mut max_y = 0;
    let mut rocks = HashSet::new();
    let mut si = 0;
    let mut y_diffs = vec![];
    let mut max_y1 = 0;
    let max_year = 2022.max(max_pattern_size * 3);
    for i in 0..max_year {
        let shape = SHAPES[i % SHAPES.len()];
        let mut x = 2;
        for y in (0..max_y+4).rev() {
            let jet = jet_patterns[si % jet_patterns.len()];
            si += 1;
            if !test_shape(&shape, &rocks, (x, y), (jet, 0)) {
                x = (x as isize + jet) as usize;
            }
            if y == 0 || test_shape(&shape, &rocks, (x, y), (0, -1)) {
                let last_max_y = max_y;
                for (j, b) in shape.iter().enumerate() {
                    if *b {
                        let x = x + j % 4;
                        let y = y + (3 - j / 4);
                        max_y = max_y.max(y+1);
                        if i <= 2021 {
                            max_y1 = max_y
                        }
                        rocks.insert((x, y));
                    }
                }
                let d = max_y - last_max_y;
                y_diffs.push(d);
                break;
            }
        }
        //print_rocks(&rocks);
    }
    let p = find_pattern(&y_diffs, min_pattern_size, max_pattern_size);
    let mut max_y2 = 0;
    if let Some((start, end)) = p {
        let pf = (1000000000000 - max_year) / (end - start);
        let pd = (1000000000000 - max_year) % (end - start);
        max_y2 = max_y + pf * y_diffs[start..end].iter().sum::<usize>()
            + y_diffs[start..start + pd].iter().sum::<usize>();
    }
    (max_y1, max_y2)
}

fn main() {
    println!("{:?}", run("input/17-example.txt", 10, 100));
    println!("{:?}", run("input/17.txt", 3000, 4000));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_17() {
        let (first, second) = super::run("input/17-example.txt", 10, 100);
        assert_eq!(first, 3068);
        assert_eq!(second, 1514285714288);
    }

    #[test]
    fn test_17() {
        let (first, second) = super::run("input/17.txt", 3000, 4000);
        assert_eq!(first, 3232);
        assert_eq!(second, 1585632183915);
    }
}
