use std::io;
use std::iter::FromIterator;

fn run(vec: &mut Vec<i32>) -> i32 {
    let mut pc: usize = 0;
    loop {
        let mut read = || {
            let val = vec[pc];
            pc += 1;
            val as usize
        };
        let op = read();
        match op {
            1 => {
                let op1 = read();
                let op2 = read();
                let dest = read();
                vec[dest] = vec[op1] + vec[op2];
            },
            2 => {
                let op1 = read();
                let op2 = read();
                let dest = read();
                vec[dest] = vec[op1] * vec[op2];
            },
            99 => break,
            _ => (),
        }
    }
    vec[0]
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Failed to read line");
    let mut vec: Vec<i32> = Vec::from_iter(line.trim().split(",")
        .map(|x| x.parse().expect("Failed to parse"))
    );
    vec[1] = 12;
    vec[2] = 2;
    {
        let result = run(&mut vec.clone());
        println!("{}", result);
    }
    'noun_loop: for noun in 0..99 {
        for verb in 0..99 {
            vec[1] = noun;
            vec[2] = verb;
            let result = run(&mut vec.clone());
            if result == 19690720 {
                println!("{:02}{:02}", noun, verb);
                break 'noun_loop;
            }
        }
    }
}
