use std::fs::File;
use std::io::{BufRead, BufReader};

/// get the (lose, win) shapes for the given (opponent, player) shapes
fn get_shapes(a: usize, b: usize) -> (usize, usize) {
    (
        {
            let v = a as isize - 1;
            if v < 0 {
                2
            } else {
                v % 3
            }
        } as usize, // lose value
        (a + 1) % 3,
    ) // win value
}

/// calculate score for the given (opponent, player) shapes
fn calc_score(a: usize, b: usize) -> usize {
    let (l, w) = get_shapes(a, b);
    (match b {
        _ if a == b => 3, // draw
        _ if l == b => 0, // lose
        _ if w == b => 6, // win
        _ => panic!("unhandled a value"),
    } + (b + 1))
}

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut score = 0;
    let mut score2 = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let (a, b) = (line.bytes().nth(0).unwrap(), line.bytes().nth(2).unwrap());
        let a = (a - b'A') as usize;
        let b = (b - b'X') as usize;
        score += calc_score(a, b);
        let (l, w) = get_shapes(a, b);
        score2 += match b {
            0 => calc_score(a, l), // lose
            1 => calc_score(a, a), // draw
            2 => calc_score(a, w), // win
            _ => panic!("unhandled value"),
        };
    }
    (score, score2)
}

fn main() {
    println!("{:?}", run("input/02-example.txt"));
    println!("{:?}", run("input/02.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_02() {
        let (first, second) = super::run("input/02-example.txt");
        assert_eq!(first, 15);
        assert_eq!(second, 12);
    }

    #[test]
    fn test_02() {
        let (first, second) = super::run("input/02.txt");
        assert_eq!(first, 12276);
        assert_eq!(second, 9975);
    }
}
