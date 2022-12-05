use std::fs::File;
use std::io::{BufRead, BufReader, Read};

fn _run(filename: &str, crate_mover_9001: bool) -> String {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut stacks: Vec<Vec<char>> = vec![];
    for line in reader.by_ref().lines() {
        let line = line.unwrap();
        let stack_len = (line.len() + 1) / 4;
        if stack_len == 0 {
            break;
        }
        if stacks.len() == 0 {
            for i in 0..stack_len {
                stacks.push(vec![]);
            }
        }
        for i in (1..line.len()).step_by(4) {
            let c = line.chars().nth(i).unwrap();
            if c.is_alphabetic() {
                let stack_i = i / 4;
                stacks[stack_i].push(c);
            }
        }
    }
    // reverse so last item is on top
    for stack in &mut stacks {
        stack.reverse();
    }
    for line in reader.lines() {
        let line = line.unwrap();
        let mut split = line.split_whitespace();
        split.next().unwrap();
        let count: usize = split.next().unwrap().parse().unwrap();
        split.next().unwrap();
        let from: usize = split.next().unwrap().parse().unwrap();
        split.next().unwrap();
        let to: usize = split.next().unwrap().parse().unwrap();
        let stack_len = stacks[from - 1].len();
        let mut c: Vec<char> = stacks[from - 1].drain(stack_len - count..).collect();
        let to_stack = &mut stacks[to - 1];
        if !crate_mover_9001 {
            c.reverse();
        }
        for i in c {
            to_stack.push(i);
        }
    }
    let mut msg = String::new();
    for stack in &stacks {
        msg.push(*stack.last().unwrap());
    }
    msg
}

fn run(filename: &str) -> (String, String) {
    (_run(filename, false), _run(filename, true))
}

fn main() {
    println!("{:?}", run("input/05-example.txt"));
    println!("{:?}", run("input/05.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_05() {
        let (first, second) = super::run("input/05-example.txt");
        assert_eq!(first, "CMZ");
        assert_eq!(second, "MCD");
    }

    #[test]
    fn test_05() {
        let (first, second) = super::run("input/05.txt");
        assert_eq!(first, "SPFMVDTZT");
        assert_eq!(second, "ZFSJBPRFP");
    }
}
