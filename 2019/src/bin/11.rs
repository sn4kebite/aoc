use std::io;
use std::collections::HashMap;
use std::iter::FromIterator;
use std::sync::mpsc;
use std::thread;

fn run_intcode(vec: &mut Vec<i64>, sender: mpsc::Sender<i64>, receiver: mpsc::Receiver<i64>) {
    let mut pc: usize = 0;
    let mut base: i64 = 0;
    loop {
        let mut read = || {
            let val = vec[pc];
            pc += 1;
            val
        };
        let parse_input = |mode, val| {
            match mode {
                0 => vec[val as usize],
                1 => val as i64,
                2 => vec[(base + val) as usize],
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
                vec[dest] = val1 + val2;
            },
            2 => { // multiply
                let val1 = parse_input(mode1, read());
                let val2 = parse_input(mode2, read());
                let dest = parse_output(mode3, read());
                vec[dest] = val1 * val2;
            },
            3 => { // input
                let dest = parse_output(mode1, read());
                //println!("reading input");
                let input = receiver.recv().expect("Failed to receive tile");
                vec[dest] = input;
                //println!("@{} = {}", dest, input);
            },
            4 => { // output
                let val = parse_input(mode1, read());
                //println!("Output: {}", val);
                sender.send(val).expect("Failed to send value");
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
                vec[dest] = {
                    if val1 < val2 {
                        1
                    } else {
                        0
                    }
                };
            },
            8 => { // jump if val1 == val2
                let val1 = parse_input(mode1, read());
                let val2 = parse_input(mode2, read());
                let dest = parse_output(mode3, read());
                vec[dest] = {
                    if val1 == val2 {
                        1
                    } else {
                        0
                    }
                };
            },
            9 => { // change base
                let val1 = parse_input(mode1, read());
                base += val1;
            },
            99 => break,
            _ => panic!("Invalid opcode {} at {}", op, pc-1),
        }
    }
}

fn run(mut vec: &mut Vec<i64>, start_white: bool) {
    let (thread_tx, rx) = mpsc::channel();
    let (tx, thread_rx) = mpsc::channel();
    let mut hull = HashMap::new();
    if start_white {
        hull.insert((0, 0), 1);
    }
    let thread = thread::spawn(move || {
        let mut direction = 0;
        let mut position = (0, 0);
        let mut min_x = std::i32::MAX;
        let mut min_y = std::i32::MAX;
        let mut max_x = std::i32::MIN;
        let mut max_y = std::i32::MIN;
        loop {
            match tx.send(*hull.get(&position).or(Some(&0))
                    .expect("hull")) {
                Err(_) => break,
                _ => (),
            }
            let val = rx.recv().expect("Failed to receive paint color");
            hull.insert(position, val);
            let turn = rx.recv().expect("Failed to receive turn");
            direction = (direction + (turn * 2 - 1)) % 4;
            if direction < 0 {
                direction += 4;
            }
            //println!("turn {} {} @ {:?}", turn, direction, position);
            match direction {
                0 => position.1 -= 1,
                1 => position.0 += 1,
                2 => position.1 += 1,
                3 => position.0 -= 1,
                _ => (),
            };
            min_x = std::cmp::min(min_x, position.0);
            min_y = std::cmp::min(min_y, position.1);
            max_x = std::cmp::max(max_x, position.0);
            max_y = std::cmp::max(max_y, position.1);
        }
        if start_white {
            for y in min_y..max_y+1 {
                for x in min_x..max_x+1 {
                    print!("{}", match hull.get(&(x, y)).or(Some(&0)).expect("hull") {
                        1 => '#',
                        _ => ' ',
                    });
                }
                println!("");
            }
        } else {
            println!("Painted: {}", hull.len());
        }
    });
    run_intcode(&mut vec, thread_tx, thread_rx);
    thread.join().expect("join");
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Failed to read line");
    let mut vec: Vec<i64> = Vec::from_iter(line.trim().split(",")
        .map(|x| x.parse().expect("Failed to parse"))
    );
    vec.resize(1200, 0);
    run(&mut vec, false);
    run(&mut vec, true);
}
