use std::io;
use std::iter::FromIterator;
use std::collections::HashSet;
use std::sync::mpsc;
use std::thread;

//fn run(vec: &mut Vec<i32>, input1: i32, input2: i32) -> i32 {
fn run(vec: &mut Vec<i32>, phase: i32, rx: mpsc::Receiver<i32>, tx: mpsc::Sender<i32>) -> i32 {
    let mut pc: usize = 0;
    let mut input_sent = false;
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
            3 => { // input
                let dest = read();
                vec[dest] = {
                    if input_sent {
                        rx.recv().expect(&format!("Failed to receive input on phase {}", phase))
                    } else {
                        input_sent = true;
                        phase
                    }
                };
                //println!("Input {} on phase {}", vec[dest], phase);
            },
            4 => { // output
                output = read_arg(mode1);
                match tx.send(output) {
                    Err(_) => return output,
                    _ => (),
                };
                //println!("Output {} from phase {}", output, phase);
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
    //vec[4]
    //println!("Exiting phase {}", phase);
    output
}

fn run_part1(vec: &Vec<i32>, a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
    let (base_tx, mut rx) = mpsc::channel();
    let mut threads = Vec::new();
    for i in [a, b, c, d, e].iter() {
        let i = *i;
        let (new_tx, new_rx) = mpsc::channel();
        let mut amp = vec.clone();
        threads.push(thread::spawn(move || {
            run(&mut amp, i, rx, new_tx);
        }));
        rx = new_rx;
    }
    base_tx.send(0).expect("base_tx");
    while threads.len() > 0 {
        let thread = threads.pop();
        thread.expect("thread").join().expect("join");
    }
    return rx.recv().expect("last_rx");
}

fn run_part2(vec: &Vec<i32>, a: i32, b: i32, c: i32, d: i32, e: i32) -> i32 {
    let (base_tx, mut rx) = mpsc::channel();
    let mut threads = Vec::new();
    for i in [a, b, c, d].iter() {
        let i = *i;
        let (new_tx, new_rx) = mpsc::channel();
        let mut amp = vec.clone();
        threads.push(thread::spawn(move || {
            run(&mut amp, i, rx, new_tx);
        }));
        rx = new_rx;
    }
    base_tx.send(0).expect("base_tx");
    let (last_tx, last_rx) = mpsc::channel();
    {
        let mut amp = vec.clone();
        threads.push(thread::spawn(move || {
            last_tx.send(run(&mut amp, e, rx, base_tx)).expect("last_tx");
        }));
    }
    while threads.len() > 0 {
        let thread = threads.pop();
        thread.expect("thread").join().expect("join");
    }
    return last_rx.recv().expect("last_rx");
}

fn main() {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Failed to read line");
    let mut vec: Vec<i32> = Vec::from_iter(line.trim().split(",")
        .map(|x| x.parse().expect("Failed to parse"))
    );
    while vec.len() < 100000 {
        vec.push(0);
    }
    let mut largest_output1 = 0;
    let mut largest_output2 = 0;
    for a in 0..5 {
        let mut visited = HashSet::new();
        visited.insert(a);
        for b in 0..5 {
            if visited.contains(&b) {
                continue;
            }
            let mut visited = visited.clone();
            visited.insert(b);
            for c in 0..5 {
                if visited.contains(&c) {
                    continue;
                }
                let mut visited = visited.clone();
                visited.insert(c);
                for d in 0..5 {
                    if visited.contains(&d) {
                        continue;
                    }
                    let mut visited = visited.clone();
                    visited.insert(d);
                    for e in 0..5 {
                        if visited.contains(&e) {
                            continue;
                        }
                        largest_output1 = std::cmp::max(largest_output1, run_part1(&vec, a, b, c, d, e));
                        largest_output2 = std::cmp::max(largest_output2, run_part2(&vec, a+5, b+5, c+5, d+5, e+5));
                    }
                }
            }
        }
    }
    println!("Largest output 1: {}", largest_output1);
    println!("Largest output 2: {}", largest_output2);
}
