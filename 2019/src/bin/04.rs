use std::io;
use std::iter::FromIterator;

fn is_valid(s: &String, check_adjecent: bool) -> bool {
    let mut last = '/';
    let mut has_adjacent = false;
    let mut adjacent = false;
    let mut group = false;
    for c in s.chars() {
        if c == last {
            if group && check_adjecent {
                adjacent = false;
            } else {
                adjacent = true;
            }
            group = true;
        } else {
            if adjacent {
                has_adjacent = true;
            }
            group = false;
        }
        if c < last {
            return false;
        }
        last = c;
    }
    has_adjacent || adjacent
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Failed to read line");
    let args: Vec<i32> = Vec::from_iter(line.trim().split("-")
        .map(|x| x.parse().expect("Failed to parse input")));
    let mut valid1 = 0;
    let mut valid2 = 0;
    for input in args[0]..args[1]+1 {
        if is_valid(&input.to_string(), false) {
            valid1 += 1;
        }
        if is_valid(&input.to_string(), true) {
            valid2 += 1;
        }
    }
    println!("Valid: {}", valid1);
    println!("Valid: {}", valid2);
}
