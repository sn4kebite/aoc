use std::io;
use std::iter::FromIterator;

fn run(vec: &mut Vec<i32>, input: i32) -> i32 {
    let mut pc: usize = 0;
    let mut output = 0;
    loop {
        let mut read = || {
            let val = vec[pc];
            pc += 1;
            val as usize
        };
        let mut op = read();
        let mode1 = {
            if op >= 100 {
                op / 100 % 10
            } else {
                0
            }
        };
        let mode2 = {
            if op >= 1000 {
                op / 1000 % 10
            } else {
                0
            }
        };
        let mut read_arg = |mode| {
            //let val = vec[pc];
            //pc += 1;
            let val = read();
            if mode == 1 {
                return val as i32;
            }
            vec[val as usize]
        };
        op = op % 100;
        //println!("Run op {}", op);
        match op {
            1 => {
                let val1 = read_arg(mode1);
                let val2 = read_arg(mode2);
                let dest = read();
                //println!("{} + {} = {}@{}", val1, val2, val1 + val2, dest);
                vec[dest] = val1 + val2;
            },
            2 => {
                let val1 = read_arg(mode1);
                let val2 = read_arg(mode2);
                let dest = read();
                vec[dest] = val1 * val2;
            },
            3 => {
                let dest = read();
                vec[dest] = input;
                println!("@{} = {}", dest, input);
            },
            4 => {
                output = read_arg(mode1);
                println!("Output: {}", output);
            },
            5 => {
                let val1 = read_arg(mode1);
                let val2 = read_arg(mode2);
                if val1 != 0 {
                    pc = val2 as usize;
                }
            },
            6 => {
                let val1 = read_arg(mode1);
                let val2 = read_arg(mode2);
                if val1 == 0 {
                    pc = val2 as usize;
                }
            },
            7 => {
                let val1 = read_arg(mode1);
                let val2 = read_arg(mode2);
                let val3 = read();
                vec[val3] = {
                    if val1 < val2 {
                        1
                    } else {
                        0
                    }
                };
            },
            8 => {
                let val1 = read_arg(mode1);
                let val2 = read_arg(mode2);
                let val3 = read();
                vec[val3] = {
                    if val1 == val2 {
                        1
                    } else {
                        0
                    }
                };
            },
            99 => break,
            _ => (),
        }
    }
    output
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Failed to read line");
    let mut vec: Vec<i32> = Vec::from_iter(line.trim().split(",")
        .map(|x| x.parse().expect("Failed to parse"))
    );
    while vec.len() < 1106 {
        vec.push(0);
    }
    //vec[1] = 12;
    //vec[2] = 2;
    println!("Result: {}", run(&mut vec.clone(), 1));
    println!("Result: {}", run(&mut vec.clone(), 5));
    /*'noun_loop: for noun in 0..99 {
        for verb in 0..99 {
            vec[1] = noun;
            vec[2] = verb;
            let result = run(&mut vec.clone());
            if result == 19690720 {
                println!("{:02}{:02}", noun, verb);
                break 'noun_loop;
            }
        }
    }*/
}
