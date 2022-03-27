use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn organize_amphipods(hallway: &[char; 11], rooms: &[char; 16], room_size: usize) -> usize {
    let mut states = HashSet::new();
    let mut queue = BinaryHeap::new();
    let room_count = 4;
    queue.push(Reverse((0, hallway.clone(), rooms.clone())));
    while let Some(Reverse((cost, hallway, rooms))) = queue.pop() {
        if rooms[0..room_size * room_count]
            .iter()
            .enumerate()
            .all(|(room, c)| {
                let room_type = (b'A' + (room / room_size) as u8) as char;
                *c == room_type
            })
        {
            return cost;
        }
        if states.contains(&(hallway, rooms)) {
            continue;
        }
        states.insert((hallway.clone(), rooms.clone()));
        for room in 0..room_count {
            let room_type = (b'A' + (room as u8)) as char;
            if rooms[room * room_size..(room + 1) * room_size]
                .iter()
                .all(|c| *c == room_type)
            {
                continue;
            }
            // room position in the hallway
            let room_pos = 2 + room * 2;
            // 0=outer, room_size=inner
            for pos in 0..room_size {
                if !rooms[room * room_size + pos].is_alphabetic() {
                    continue;
                }
                let mut rooms = rooms.clone();
                let c = rooms[room * room_size + pos];
                rooms[room * room_size + pos] = '.';
                for i in [0, 1, 3, 5, 7, 9, 10] {
                    let mut start = room_pos;
                    let mut end = i;
                    if start > end {
                        let temp = start;
                        start = end;
                        end = temp + 1;
                    } else {
                        end = end + 1;
                    }
                    if hallway[start..end].iter().all(|t| *t == '.') {
                        let mut hallway = hallway.clone();
                        hallway[i] = c;
                        let cost = cost
                            + 10_usize.pow((c as u8 - b'A') as u32)
                                * ((end - start - 1) as usize + pos + 1);
                        queue.push(Reverse((cost, hallway, rooms)));
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
            for pos in 0..room_size {
                // reverse so we start with the inner tile
                let pos = room_size - 1 - pos;
                // Skip rooms we can't move to
                match rooms[room * room_size + pos] {
                    //'A'..'D' if rooms[room * room_size + pos] != room_type => break,
                    'A' | 'B' | 'C' | 'D' if rooms[room * room_size + pos] != room_type => break,
                    'A' | 'B' | 'C' | 'D' if rooms[room * room_size + pos] == room_type => continue,
                    '.' => (),
                    _ => panic!("invalid tile value!"),
                }
                let room_pos = 2 + room * 2;
                let mut start = i;
                let mut end = room_pos;
                if start > end {
                    let temp = start;
                    start = end;
                    end = temp + 1;
                } else {
                    end = end + 1;
                }
                if hallway[start..end].iter().all(|t| *t == '.') {
                    let mut rooms = rooms.clone();
                    rooms[room * room_size + pos] = room_type;
                    let cost =
                        cost + 10_usize.pow(room as u32) * ((end - start - 1) as usize + pos + 1);
                    queue.push(Reverse((cost, hallway, rooms)));
                }
                break;
            }
        }
    }
    0
}

fn run2(filename: &str, room_size: usize) -> usize {
    let hallway = ['.'; 11];
    let mut rooms = ['.'; 16];
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
                    rooms[room * room_size + pos] = c;
                    room += 1;
                }
            }
        }
        if room_size == 4 {
            // Mutate the room for the second part
            let mut_room = b"DDCBBAAC";
            for room in 0..4 {
                rooms[room * room_size + 3] = rooms[room * room_size + 1];
                rooms[room * room_size + 1] = mut_room[room * 2 + 0] as char;
                rooms[room * room_size + 2] = mut_room[room * 2 + 1] as char;
            }
        }
    }
    organize_amphipods(&hallway, &rooms, room_size)
}

fn run(filename: &str) -> (usize, usize) {
    (run2(&filename, 2), run2(&filename, 4))
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
        assert_eq!(second, 44169);
    }

    #[test]
    fn test_input_23() {
        let (first, second) = super::run("input/23.txt");
        assert_eq!(first, 14350);
        assert_eq!(second, 49742);
    }
}
