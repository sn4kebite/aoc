use std::io;
use std::io::BufRead;

fn main() {
    let mut fuel1: i32 = 0;
    let mut fuel2: i32 = 0;
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let mass: i32 = match line.parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Failed to parse {}", line);
                continue;
            },
        };
        let mut added = mass / 3 - 2;
        fuel1 += added;
        fuel2 += added;
        // Calculation for part 2
        while added > 0 {
            added = added / 3 - 2;
            if added > 0 {
                fuel2 += added;
            }
        }
    }
    println!("{}", fuel1);
    println!("{}", fuel2);
}
