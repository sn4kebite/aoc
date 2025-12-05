use std::fs::File;
use std::io::{BufRead, BufReader};

fn neighbouring(papers: &Vec<Vec<bool>>, pos: (usize, usize)) -> usize {
    let (x, y) = pos;
    let mut n = 0;
    if x > 0 {
        if papers[y][x - 1] {
            n += 1;
        }
        if y > 0 && papers[y - 1][x - 1] {
            n += 1;
        }
        if y < papers.len() - 1 && papers[y + 1][x - 1] {
            n += 1;
        }
    }
    if x < papers[0].len() - 1 {
        if papers[y][x + 1] {
            n += 1;
        }
        if y > 0 && papers[y - 1][x + 1] {
            n += 1;
        }
        if y < papers.len() - 1 && papers[y + 1][x + 1] {
            n += 1;
        }
    }
    if y > 0 && papers[y - 1][x] {
        n += 1;
    }
    if y < papers.len() - 1 && papers[y + 1][x] {
        n += 1;
    }
    n
}

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut papers: Vec<Vec<bool>> = Vec::from_iter(
        reader
            .lines()
            .map(|line| line.unwrap().chars().map(|c| c == '@').collect()),
    );
    let mut tot = 0;
    let mut remove = vec![];
    for y in 0..papers.len() {
        for x in 0..papers[y].len() {
            if papers[y][x] && neighbouring(&papers, (x, y)) < 4 {
                tot += 1;
                remove.push((x, y));
            }
        }
    }
    let mut tot2 = tot;
    while !remove.is_empty() {
        for (x, y) in &remove {
            papers[*y][*x] = false;
        }
        remove.clear();
        for y in 0..papers.len() {
            for x in 0..papers[y].len() {
                if papers[y][x] && neighbouring(&papers, (x, y)) < 4 {
                    tot2 += 1;
                    remove.push((x, y));
                }
            }
        }
    }
    (tot, tot2)
}

fn main() {
    println!("{:?}", run("input/04-example.txt"));
    println!("{:?}", run("input/04.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_04() {
        let (first, second) = super::run("input/04-example.txt");
        assert_eq!(first, 13);
        assert_eq!(second, 43);
    }

    #[test]
    fn test_04() {
        let (first, second) = super::run("input/04.txt");
        assert_eq!(first, 1445);
        assert_eq!(second, 8317);
    }
}
