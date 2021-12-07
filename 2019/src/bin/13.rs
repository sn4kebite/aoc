use std::io;
use std::collections::HashSet;
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

fn run(mut vec: &mut Vec<i64>) {
    let (thread_tx, rx) = mpsc::channel();
    let (tx, thread_rx) = mpsc::channel();
    let thread = thread::spawn(move || {
        let mut set = HashSet::new();
        let mut paddle = (0, 0);
        let mut score = 0;
        loop {
            let x = match rx.recv() {
                Ok(val) => val,
                Err(_) => break,
            };
            let y = rx.recv().expect("Failed to receive y");
            let tile = rx.recv().expect("Failed to receive tile");
            if x == -1 && y == 0 {
                score = tile;
                continue;
            }
            if tile == 2 {
                set.insert((x, y));
            } else if tile == 3 {
                paddle = (x, y);
                //println!("paddle at {},{}", x, y);
            } else if tile == 4 {
                //println!("ball at {},{}", x, y);
                if x < paddle.0 {
                    //println!("paddle left");
                    tx.send(-1).ok();
                } else if x > paddle.0 {
                    //println!("paddle right");
                    tx.send(1).ok();
                } else {
                    //println!("paddle stop");
                    tx.send(0).ok();
                }
            }
        }
        println!("{} block tiles", set.len());
        println!("score {}", score);
    });
    vec[0] = 2;
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
    vec.resize(2500, 0);
    run(&mut vec);
}
