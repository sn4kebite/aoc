use std::fs::File;
use std::io::{BufRead, BufReader};

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut trees: Vec<Vec<u8>> = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        trees.push(line.bytes().collect());
    }
    let height = trees.len();
    let width = trees[0].len();
    let mut visible = width * 2 + (height - 2) * 2;
    let mut max_score = 0;
    for x in 1..width - 1 {
        for y in 1..height - 1 {
            let tree = trees[y][x];
            if trees[y][0..x].iter().all(|v| *v < tree)
                || trees[y][x + 1..width].iter().all(|v| *v < tree)
                || trees[0..y].iter().all(|i| i[x] < tree)
                || trees[y + 1..height].iter().all(|i| i[x] < tree)
            {
                visible += 1;
            }
            let mut sum = 0;
            for i in (0..x).rev() {
                sum += 1;
                if trees[y][i] >= tree {
                    break;
                }
            }
            let mut score = sum;
            sum = 0;
            for i in x + 1..width {
                sum += 1;
                if trees[y][i] >= tree {
                    break;
                }
            }
            score *= sum;
            sum = 0;
            for i in (0..y).rev() {
                sum += 1;
                if trees[i][x] >= tree {
                    break;
                }
            }
            score *= sum;
            sum = 0;
            for i in y + 1..height {
                sum += 1;
                if trees[i][x] >= tree {
                    break;
                }
            }
            score *= sum;
            if score > max_score {
                max_score = score;
            }
        }
    }
    (visible, max_score)
}

fn main() {
    println!("{:?}", run("input/08-example.txt"));
    println!("{:?}", run("input/08.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_08() {
        let (first, second) = super::run("input/08-example.txt");
        assert_eq!(first, 21);
        assert_eq!(second, 8);
    }

    #[test]
    fn test_08() {
        let (first, second) = super::run("input/08.txt");
        assert_eq!(first, 1840);
        assert_eq!(second, 405769);
    }
}
