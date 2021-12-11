use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut hi_seat_id = 0;
    let mut ids = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let mut row = 64;
        let mut col = 4;
        for (i, c) in line.chars().take(7).enumerate() {
            match c {
                'F' => row -= 2usize.pow(5u32.saturating_sub(i as u32)),
                'B' => row += 2usize.pow(5u32.saturating_sub(i as u32)),
                _ => panic!("Invalid character {}", c),
            }
        }
        if line.chars().nth(6).unwrap() == 'B' {
            row -= 1;
        }
        for (i, c) in line.chars().skip(7).enumerate() {
            match c {
                'L' => col -= 2usize.pow(1u32.saturating_sub(i as u32)),
                'R' => col += 2usize.pow(1u32.saturating_sub(i as u32)),
                _ => panic!("Invalid character {}", c),
            }
        }
        if line.chars().last().unwrap() == 'R' {
            col -= 1;
        }
        let this_id = row * 8 + col;
        ids.push(this_id);
        if this_id > hi_seat_id {
            hi_seat_id = this_id;
        }
    }
    ids.sort();
    let mut seat_id = 0;
    for slice in ids.windows(2) {
        let (a, b) = (slice[0], slice[1]);
        if b > a + 1 {
            seat_id = a + 1;
            break;
        }
    }
    (hi_seat_id, seat_id)
}

fn main() {
    println!("{:?}", run("input/05-example1.txt"));
    println!("{:?}", run("input/05-example2.txt"));
    println!("{:?}", run("input/05-example3.txt"));
    println!("{:?}", run("input/05-example4.txt"));
    println!("{:?}", run("input/05.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_05() {
        let (first, _) = super::run("input/05-example1.txt");
        assert_eq!(first, 357);
        let (first, _) = super::run("input/05-example2.txt");
        assert_eq!(first, 567);
        let (first, _) = super::run("input/05-example3.txt");
        assert_eq!(first, 119);
        let (first, _) = super::run("input/05-example4.txt");
        assert_eq!(first, 820);
    }

    #[test]
    fn test_input_05() {
        let (first, second) = super::run("input/05.txt");
        assert_eq!(first, 885);
        assert_eq!(second, 623);
    }
}
