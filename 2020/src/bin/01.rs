use std::io;
use std::io::BufRead;

fn main() {
    let mut expenses: Vec<i32> = vec![];
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let v: i32 = match line.parse() {
            Ok(num) => num,
            Err(_) => panic!("Failed to parse {}", line),
        };
        expenses.push(v);
    }
    for i in 0..expenses.len() {
        let a = expenses[i];
        for j in i + 1..expenses.len() {
            let b = expenses[j];
            if a + b == 2020 {
                println!("1: {}", a * b);
            }
            for k in j + 1..expenses.len() {
                let c = expenses[k];
                if a + b + c == 2020 {
                    println!("2: {}", a * b * c);
                }
            }
        }
    }
}
