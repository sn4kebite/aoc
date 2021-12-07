use std::cmp::Ordering;
use std::io;
use std::collections::{HashMap, HashSet, VecDeque, BinaryHeap};
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
    println!("Exiting intcode thread");
}

#[derive(Debug)]
struct Step {
    pos: (i32, i32),
    dest: (i32, i32),
    vec: Vec<i32>,
}

impl Step {
    fn distance(&self) -> i32 {
        (self.pos.0 - self.dest.0).abs() + (self.pos.1 - self.dest.1).abs()
    }
}

impl Ord for Step {
    fn cmp(&self, other: &Self) -> Ordering {
        self.distance().cmp(&other.distance())
    }
}

impl PartialOrd for Step {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Step {
    fn eq(&self, other: &Self) -> bool {
        self.distance() == other.distance()
    }
}

impl Eq for Step {}

fn find_shortest(map: &HashMap<(i32, i32), i32>, from: (i32, i32), to: (i32, i32)) -> i32 {
    let mut queue = BinaryHeap::new();
    let mut state = HashSet::new();
    state.insert(from);
    queue.push(Step{pos: (from.0, from.1-1), dest: to, vec: vec![1]});
    queue.push(Step{pos: (from.0, from.1+1), dest: to, vec: vec![2]});
    queue.push(Step{pos: (from.0-1, from.1), dest: to, vec: vec![3]});
    queue.push(Step{pos: (from.0+1, from.1), dest: to, vec: vec![4]});
    while !queue.is_empty() {
        let step = queue.pop().unwrap();
        if state.contains(&step.pos) {
            continue;
        }
        state.insert(step.pos);
        let tile = match map.get(&step.pos) {
            Some(v) => *v,
            None => -1,
        };
        if step.pos == to {
            return step.vec.len() as i32;
        }
        if tile != 1 {
            continue;
        }
        queue.push(Step{pos: (step.pos.0, step.pos.1-1), dest: to, vec: { let mut vec = step.vec.clone(); vec.push(1); vec}});
        queue.push(Step{pos: (step.pos.0, step.pos.1+1), dest: to, vec: { let mut vec = step.vec.clone(); vec.push(2); vec}});
        queue.push(Step{pos: (step.pos.0-1, step.pos.1), dest: to, vec: { let mut vec = step.vec.clone(); vec.push(3); vec}});
        queue.push(Step{pos: (step.pos.0+1, step.pos.1), dest: to, vec: { let mut vec = step.vec.clone(); vec.push(4); vec}});
    }
    0
}

fn find_next_queue(state: &mut HashSet<(i32, i32)>, mut path: &mut Vec<i32>, map: &HashMap<(i32, i32), i32>, position: &(i32, i32)) -> i32 {
    let mut queue = VecDeque::new();
    state.insert(*position);
    queue.push_back(((position.0, position.1-1), vec![1]));
    queue.push_back(((position.0, position.1+1), vec![2]));
    queue.push_back(((position.0-1, position.1), vec![3]));
    queue.push_back(((position.0+1, position.1), vec![4]));
    while !queue.is_empty() {
        let (pos, vec) = queue.pop_front().unwrap();
        if state.contains(&pos) {
            continue;
        }
        state.insert(pos);
        let tile = match map.get(&pos) {
            Some(v) => *v,
            None => -1,
        };
        if tile == 0 {
            continue;
        }
        if tile == -1 {
            println!("target {:?} with path {:?}", pos, vec);
            path.extend(vec);
            return 1;
        }
        queue.push_back(((pos.0, pos.1-1), { let mut vec = vec.clone(); vec.push(1); vec }));
        queue.push_back(((pos.0, pos.1+1), { let mut vec = vec.clone(); vec.push(2); vec }));
        queue.push_back(((pos.0-1, pos.1), { let mut vec = vec.clone(); vec.push(3); vec }));
        queue.push_back(((pos.0+1, pos.1), { let mut vec = vec.clone(); vec.push(4); vec }));
    }
    0
}

fn find_next_recursive(mut state: &mut HashSet<(i32, i32)>, mut path: &mut Vec<i32>, map: &HashMap<(i32, i32), i32>, position: &(i32, i32)) -> i32 {
    if state.contains(position) {
        return 0;
    }
    state.insert(*position);
    //println!("find_next at {:?}", position);
    let tile = *match map.get(position) {
        Some(v) => v,
        None => {
            //println!("Found unknown at {:?}", position);
            return 1
        },
    };
    //println!("tile {} at {:?}", tile, position);
    if tile == 0 {
        return 0;
    }
    if find_next_recursive(&mut state, &mut path, &map, &(position.0, position.1-1)) != 0 {
        path.push(1);
        return 1;
    }
    if find_next_recursive(&mut state, &mut path, &map, &(position.0, position.1+1)) != 0 {
        path.push(2);
        return 2;
    }
    if find_next_recursive(&mut state, &mut path, &map, &(position.0-1, position.1)) != 0 {
        path.push(3);
        return 3;
    }
    if find_next_recursive(&mut state, &mut path, &map, &(position.0+1, position.1)) != 0 {
        path.push(4);
        return 4;
    }
    0
}

fn print_maze(map: &HashMap<(i32, i32), i32>, position: &(i32, i32)) {
    static mut last_minx: i32 = std::i32::MAX;
    static mut last_miny: i32 = std::i32::MAX;
    static mut last_maxx: i32 = std::i32::MIN;
    static mut last_maxy: i32 = std::i32::MIN;
    let mut minx = std::i32::MAX;
    let mut miny = std::i32::MAX;
    let mut maxx = std::i32::MIN;
    let mut maxy = std::i32::MIN;
    for k in map.keys() {
        minx = std::cmp::min(minx, k.0);
        miny = std::cmp::min(miny, k.1);
        maxx = std::cmp::max(maxx, k.0);
        maxy = std::cmp::max(maxy, k.1);
    }
    let mut s = String::new();
    s += "\x1b[1;1H";
    unsafe {
        if last_minx != minx || last_miny != miny || last_maxx != maxx || last_maxy != maxy {
            last_minx = minx;
            last_miny = miny;
            last_maxx = maxx;
            last_maxy = maxy;
            s += "\x1b[2J";
        }
    }
    for y in miny..maxy+1 {
        for x in minx..maxx+1 {
            if (x, y) == *position {
                s += "D";
                continue;
            }
            s += match map.get(&(x, y)) {
                Some(value) => match value {
                    0 => "#",
                    1 => ".",
                    2 => "O",
                    _ => "?",
                },
                None => " ",
            };
        }
        s += "\n";
    }
    println!("{}", s);
}

fn run(mut vec: &mut Vec<i64>) {
    let (thread_tx, rx) = mpsc::channel();
    let (tx, thread_rx) = mpsc::channel();
    let thread = thread::spawn(move || {
        let mut map = HashMap::new();
        let mut position = (0, 0);
        map.insert(position, 1);
        loop {
            let mut state = HashSet::new();
            let mut path = Vec::new();
            println!("");
            find_next_queue(&mut state, &mut path, &map, &position);
            if path.is_empty() {
                print_maze(&map, &position);
                println!("Empty path!");
                break;
            }
            //path.reverse();
            for direction in path {
                //std::thread::sleep(std::time::Duration::from_millis(1));
                let next = match direction {
                    1 => (position.0, position.1-1),
                    2 => (position.0, position.1+1),
                    3 => (position.0-1, position.1),
                    4 => (position.0+1, position.1),
                    _ => panic!("Invalid direction"),
                };
                tx.send(direction as i64).expect("Failed to send command");
                match rx.recv().expect("Failed to receive status") {
                    0 => {
                        map.insert(next, 0);
                        println!("Wall at {:?}", next);
                        print_maze(&map, &position);
                        break;
                    },
                    1 => {
                        position = next;
                        map.insert(position, 1);
                        print_maze(&map, &position);
                        println!("Moved to {:?}", next);
                    },
                    2 => {
                        position = next;
                        map.insert(position, 2);
                        print_maze(&map, &position);
                        println!("Shortest path to oxygen is {} tiles", find_shortest(&map, (0, 0), position));
                        break;
                    },
                    _ => (),
                };
            }
            //print_maze(&map, &position);
        }
        print!("\x1b[2J");
        print_maze(&map, &(99, 99));
        println!("Filling oxygen");
        let mut done = false;
        let mut step = 0;
        while !done {
            done = true;
            let mut new_map = map.clone();
            for k in map.keys() {
                if !new_map.contains_key(k) {
                    continue;
                }
                let new_tile = *new_map.get(k).unwrap();
                if new_tile != 1 {
                    continue;
                }
                if *map.get(&(k.0, k.1-1)).unwrap() == 2 || *map.get(&(k.0, k.1+1)).unwrap() == 2 ||
                    *map.get(&(k.0-1, k.1)).unwrap() == 2 || *map.get(&(k.0+1, k.1)).unwrap() == 2 {
                    new_map.insert(*k, 2);
                    done = false;
                }
            }
            print_maze(&map, &(99, 99));
            println!("Filling oxygen: {}", step);
            map = new_map;
            step += 1;
        }
        step -= 1;
        println!("Oxygen filled in {} steps", step);
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
    //vec.resize(2500, 0);
    run(&mut vec);
}
