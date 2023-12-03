use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_number(s: &str, index: usize) -> Option<(usize, &str)> {
    if let Some((start, _)) = s.char_indices().skip(index).find(|(_, c)| c.is_digit(10)) {
        let (end, _) = s
            .char_indices()
            .skip(start)
            .find(|(_, c)| !c.is_digit(10))
            .unwrap_or((s.len(), '.'));
        return Some((start, s.get(start..end).expect("substring")));
    }
    None
}

fn find_symbol(s: &str, index: usize) -> Option<(usize, bool)> {
    if let Some((start, c)) = s
        .char_indices()
        .skip(index)
        .find(|(_, c)| !c.is_digit(10) && *c != '.')
    {
        return Some((start, c == '*'));
    }
    None
}

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut numbers = Vec::new();
    let mut symbols = Vec::new();
    for (y, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let mut start = 0;
        let s = line.as_str();
        while let Some((index, s)) = find_number(s, start) {
            // x, y, number
            numbers.push((index, y, s.parse::<usize>().expect("number")));
            start = index + s.len();
        }
        start = 0;
        while let Some((index, gear)) = find_symbol(s, start) {
            // x, y, is_gear, neighbors, product
            symbols.push((index, y, gear, 0, 1));
            start = index + 1;
        }
    }
    let mut sum = 0;
    for (nx, ny, n) in &numbers {
        let end_x = nx + n.ilog10() as usize;
        for (sx, sy, gear, nc, np) in &mut symbols {
            // neighbor check
            if *sx + 1 >= *nx && *sx <= end_x + 1 && *sy + 1 >= *ny && *sy <= ny + 1 {
                sum += n;
                // if symbol is a gear, add 1 to neighbor count and update product
                if *gear {
                    *nc += 1;
                    *np *= n;
                }
                break;
            }
        }
    }
    let gear_sum = symbols
        .iter()
        // is gear and count is 2
        .filter(|s| s.2 && s.3 == 2)
        .map(|s| s.4)
        .sum();
    (sum, gear_sum)
}

fn main() {
    println!("{:?}", run("input/03-example.txt"));
    println!("{:?}", run("input/03.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_01() {
        let (first, second) = super::run("input/03-example.txt");
        assert_eq!(first, 4361);
        assert_eq!(second, 467835);
    }

    #[test]
    fn test_03() {
        let (first, second) = super::run("input/03.txt");
        assert_eq!(first, 526404);
        assert_eq!(second, 84399773);
    }
}
