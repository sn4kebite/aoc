use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn verify_line(buf: &str) -> (Option<char>, usize) {
    let mut stack: Vec<char> = vec![];
    for c in buf.chars() {
        match c {
            '(' => stack.push(')'),
            '[' => stack.push(']'),
            '{' => stack.push('}'),
            '<' => stack.push('>'),
            ')' | ']' | '}' | '>' => match stack.pop() {
                Some(v) => {
                    if v != c {
                        return (Some(c), 0);
                    }
                }
                None => panic!("stack is empty"),
            },
            _ => panic!("invalid character {}", c),
        }
    }
    let mut score = 0;
    while let Some(c) = stack.pop() {
        score = score * 5
            + match c {
                ')' => 1,
                ']' => 2,
                '}' => 3,
                '>' => 4,
                _ => panic!("invalid character"),
            };
    }
    (None, score)
}

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut illegal_score = 0;
    let mut scores = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();
        match verify_line(&line) {
            (Some(c), _) => {
                illegal_score += match c {
                    ')' => 3,
                    ']' => 57,
                    '}' => 1197,
                    '>' => 25137,
                    _ => panic!("invalid character"),
                }
            }
            (None, score) => scores.push(score),
        }
    }
    scores.sort();
    (illegal_score, scores[scores.len() / 2])
}

fn main() {
    println!("{:?}", run("input/10-example.txt"));
    println!("{:?}", run("input/10.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_10() {
        let (first, second) = super::run("input/10-example.txt");
        assert_eq!(first, 26397);
        assert_eq!(second, 288957);
    }

    #[test]
    fn test_input_10() {
        let (first, second) = super::run("input/10.txt");
        assert_eq!(first, 294195);
        assert_eq!(second, 3490802734);
    }
}
