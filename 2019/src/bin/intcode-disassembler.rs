use std::io;
use std::iter::FromIterator;

fn run(vec: &mut Vec<i64>) {
    let mut pc: usize = 0;
    let base: i64 = 0;
    loop {
        let mut read = || {
            let val = vec[pc];
            pc += 1;
            val
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
            match mode {
                0 => vec[val as usize],
                1 => val as i64,
                2 => vec[(base + val) as usize],
                _ => panic!("Invalid mode {}", mode),
            }
        };
        let format_arg = |mode, val| {
            match mode {
                0 => format!("{}", val),
                1 => format!("[{}]", val),
                2 => format!("[base + {}]", val),
                _ => panic!("Invalid mode {}", mode),
            }
        };
        op = op % 100;
        //println!("Run op {}", op);
        match op {
            1 => { // add
                let val1 = read_arg(mode1);
                let val2 = read_arg(mode2);
                let dest = read();
                println!("add: {} + {} -> {}", format_arg(mode1, val1), format_arg(mode2, val2), format_arg(0, dest));
            },
            2 => { // multiply
                let val1 = read_arg(mode1);
                let val2 = read_arg(mode2);
                let dest = read();
                println!("mul: {} x {} -> {}", format_arg(mode1, val1), format_arg(mode2, val2), format_arg(0, dest));
            },
            3 => { // input
                let dest = read_arg(mode1);
                println!("input {}", format_arg(mode1, dest));
            },
            4 => { // output
                let val = read_arg(mode1);
                println!("Output: {}", format_arg(mode1, val));
            },
            5 => { // jump if nonzero
                let val1 = read_arg(mode1);
                let val2 = read_arg(mode2);
                println!("jnz {}, {}", format_arg(mode1, val1), format_arg(mode2, val2));
            },
            6 => { // jump if zero
                let val1 = read_arg(mode1);
                let val2 = read_arg(mode2);
                println!("jz {}, {}", format_arg(mode1, val1), format_arg(mode2, val2));
            },
            7 => { // check if val1 < arg2
                let val1 = read_arg(mode1);
                let val2 = read_arg(mode2);
                let val3 = read();
                println!("cmp {} < {} -> {}", format_arg(mode1, val1), format_arg(mode2, val2), format_arg(0, val3));
            },
            8 => { // check if val1 == val2
                let val1 = read_arg(mode1);
                let val2 = read_arg(mode2);
                let val3 = read();
                println!("cmp {} == {} -> {}", format_arg(mode1, val1), format_arg(mode2, val2), format_arg(0, val3));
            },
            9 => { // change base
                let val1 = read_arg(mode1);
                println!("base {}", format_arg(mode1, val1));
            },
            99 => break,
            _ => (),
        }
    }
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Failed to read line");
    let mut vec: Vec<i64> = Vec::from_iter(line.trim().split(",")
        .map(|x| x.parse().expect("Failed to parse"))
    );
    run(&mut vec);
}
