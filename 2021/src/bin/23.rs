use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn run2(hallway: &[char; 11], rooms: &[char; 8]) -> usize {
    let mut states = HashSet::new();
    let mut queue = BinaryHeap::new();
    for room in 0..4 {
        let room_type = (b'A' + (room as u8)) as char;
        if rooms[room * 2 + 0] == room_type && rooms[room * 2 + 1] == room_type {
            continue;
        }
        if rooms[room * 2 + 0].is_alphabetic() {
            let mut hallway = hallway.clone();
            let mut rooms = rooms.clone();
            let c = rooms[room * 2 + 0];
            rooms[room * 2 + 0] = '.';
            for i in [0, 1, 3, 5, 7, 9, 10] {
                if hallway[i] == '.' {
                    hallway[i] = c;
                    queue.push(Reverse((0, hallway.clone(), rooms.clone())));
                }
            }
        }
    }
    let mut lowest_cost = None;
    while let Some(Reverse((cost, hallway, rooms))) = queue.pop() {
        //println!("{:?} {:?} {}", hallway, rooms, cost);
        if rooms.iter().enumerate().all(|(room, c)| {
            let room_type = (b'A' + (room as u8) / 2) as char;
            *c == room_type
        }) {
            println!("done with cost {}", cost);
            if lowest_cost.is_none() {
                lowest_cost = Some(cost);
                break;
            } else if let Some(c) = lowest_cost {
                if cost < c {
                    lowest_cost = Some(cost);
                }
            }
            continue;
        }
        if states.contains(&(hallway, rooms)) {
            continue;
        }
        states.insert((hallway.clone(), rooms.clone()));
        for room in 0..4 {
            let room_type = (b'A' + (room as u8)) as char;
            if rooms[room * 2 + 0] == room_type && rooms[room * 2 + 1] == room_type {
                continue;
            }
            // room position in the hallway
            let room_pos = 2 + room * 2;
            for pos in 0..2 {
                if !rooms[room * 2 + pos].is_alphabetic() {
                    continue;
                }
                let mut rooms = rooms.clone();
                let c = rooms[room * 2 + pos];
                rooms[room * 2 + pos] = '.';
                let mut next = vec![0, 1, 3, 5, 7, 9, 10];
                next.sort_by_key(|v| *v as i32 - (2 + room as i32 * 2));
                for i in next {
                    let dir: i32 = if room_pos > i { -1 } else { 1 };
                    if (dir == 1 && i == hallway.len() - 1) || (dir == -1 && i == 0) {
                        continue;
                    }
                    let mut start = room_pos;
                    let mut end = i;
                    if start > end {
                        let temp = start;
                        start = end;
                        end = temp + 1;
                    } else {
                        end = end + 1;
                    }
                    if hallway[start..end].iter().all(|c| *c == '.') {
                        let mut hallway = hallway.clone();
                        hallway[i] = c;
                        let cost = cost
                            + 10_usize.pow(room as u32)
                                * ((i as i32 - room_pos as i32).abs() as usize + pos + 1);
                        queue.push(Reverse((cost, hallway, rooms.clone())));
                    }
                }
                // we wan't move from the inner position if the outer position was occupied
                break;
            }
        }
        for (i, c) in hallway.iter().enumerate() {
            if *c == '.' {
                continue;
            }
            let mut hallway = hallway.clone();
            hallway[i] = '.';
            let room_type = *c;
            let room = (room_type as u8 - b'A') as usize;
            for pos in 0..2 {
                let pos = 1 - pos;
                if rooms[room * 2 + pos] != '.' {
                    continue;
                }
                let room_pos = 2 + room * 2;
                let dir: i32 = if room_pos < i { -1 } else { 1 };
                if (dir == 1 && i == hallway.len() - 1) || (dir == -1 && i == 0) {
                    continue;
                }
                let mut start = i;
                let mut end = room_pos;
                if start > end {
                    let temp = start;
                    start = end;
                    end = temp + 1;
                } else {
                    end = end + 1;
                }
                if hallway[start..end].iter().all(|c| *c == '.') {
                    let mut rooms = rooms.clone();
                    rooms[room * 2 + pos] = *c;
                    let cost = cost
                        + 10_usize.pow(room as u32)
                            * ((i as i32 - room_pos as i32).abs() as usize + pos + 1);
                    queue.push(Reverse((cost, hallway.clone(), rooms)));
                    break;
                }
            }
        }
    }
    println!("unique states {}", states.len());
    lowest_cost.unwrap()
}

fn run(filename: &str) -> (usize, usize) {
    let mut hallway = ['.'; 11];
    let mut rooms = ['.'; 8];
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_line(&mut buf).unwrap();
    reader.read_line(&mut buf).unwrap();
    {
        for pos in 0..2 {
            let mut buf = String::new();
            reader.read_line(&mut buf).unwrap();
            let mut room = 0;
            for c in buf.chars() {
                if c.is_alphabetic() {
                    rooms[room * 2 + pos] = c;
                    room += 1;
                }
            }
        }
    }
    println!("hallway {:?}", hallway);
    println!("rooms {:?}", rooms);
    (run2(&hallway, &rooms), 0)
}

fn main() {
    println!("{:?}", run("input/23-example.txt"));
    println!("{:?}", run("input/23.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_23() {
        let (first, second) = super::run("input/23-example.txt");
        assert_eq!(first, 12521);
        assert_eq!(second, 0);
    }

    #[test]
    fn test_input_23() {
        let (first, second) = super::run("input/23.txt");
        assert_eq!(first, 0);
        assert_eq!(second, 0);
    }
}
