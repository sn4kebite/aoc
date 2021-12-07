use std::io;
use std::iter::FromIterator;
use std::collections::HashMap;

//fn run(vec: &mut Vec<i64>) {
fn run(vec: &mut HashMap<usize, i64>, input: i64) {
    let mut pc: usize = 0;
    let mut base: i64 = 0;
    loop {
        let mut read = || {
            let val = vec[&pc];
            pc += 1;
            val
        };
        let parse_input = |mode, val| {
            match mode {
                0 => vec[&(val as usize)],
                1 => val as i64,
                2 => vec[&((base + val) as usize)],
                _ => panic!("Invalid mode {}", mode),
            }
        };
        let parse_output = |mode, val| {
            (match mode {
                0 => val,
                2 => base + val,
                _ => panic!("Invalid mode {}", mode),
            }) as usize
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
        let mode3 = {
            if op >= 10000 {
                op / 10000 % 10
            } else {
                0
            }
        };
        op = op % 100;
        match op {
            1 => { // add
                let val1 = parse_input(mode1, read());
                let val2 = parse_input(mode2, read());
                let dest = parse_output(mode3, read());
                vec.insert(dest, val1 + val2);
            },
            2 => { // multiply
                let val1 = parse_input(mode1, read());
                let val2 = parse_input(mode2, read());
                let dest = parse_output(mode3, read());
                vec.insert(dest, val1 * val2);
            },
            3 => { // input
                let dest = parse_output(mode1, read());
                let input = input;
                vec.insert(dest, input);
                //println!("@{} = {}", dest, input);
            },
            4 => { // output
                let val = parse_input(mode1, read());
                println!("Output: {}", val);
            },
            5 => { // jump if nonzero
                let val1 = parse_input(mode1, read());
                let val2 = parse_input(mode2, read());
                if val1 != 0 {
                    pc = val2 as usize;
                }
            },
            6 => { // jump if zero
                let val1 = parse_input(mode1, read());
                let val2 = parse_input(mode2, read());
                if val1 == 0 {
                    pc = val2 as usize;
                }
            },
            7 => { // jump if val1 < arg2
                let val1 = parse_input(mode1, read());
                let val2 = parse_input(mode2, read());
                let dest = parse_output(mode3, read());
                vec.insert(dest, {
                    if val1 < val2 {
                        1
                    } else {
                        0
                    }
                });
            },
            8 => { // jump if val1 == val2
                let val1 = parse_input(mode1, read());
                let val2 = parse_input(mode2, read());
                let dest = parse_output(mode3, read());
                vec.insert(dest, {
                    if val1 == val2 {
                        1
                    } else {
                        0
                    }
                });
            },
            9 => { // change base
                let val1 = parse_input(mode1, read());
                base += val1;
            },
            99 => break,
            _ => panic!("Invalid opcode {}", op),
        }
    }
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Failed to read line");
    //let mut vec: Vec<i64> = Vec::from_iter(line.trim().split(",")
    //    .map(|x| x.parse().expect("Failed to parse"))
    //);
    //let mut map: HashMap<usize, i64> = HashMap::from_iter(vec.iter().enumerate().map(
    let mut map: HashMap<usize, i64> = HashMap::from_iter(line.trim().split(",").enumerate().map(
        |item| (item.0, item.1.parse().expect("Failed to parse string as i64"))
    ));
    run(&mut map, 1);
    run(&mut map, 2);
}
