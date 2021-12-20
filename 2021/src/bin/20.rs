use std::collections::{HashMap, HashSet, VecDeque};
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn print_map(map: &HashMap<(i32, i32), bool>) {
    let minx = *map.keys().map(|(x, _)| x).min().unwrap();
    let maxx = *map.keys().map(|(x, _)| x).max().unwrap();
    let miny = *map.keys().map(|(_, y)| y).min().unwrap();
    let maxy = *map.keys().map(|(_, y)| y).max().unwrap();
    println!("({},{}) → ({},{})", minx, miny, maxx, maxy);
    for y in miny..maxy + 1 {
        for x in minx..maxx + 1 {
            print!(
                "{}",
                if *map.get(&(x, y)).unwrap_or(&false) {
                    '#'
                } else {
                    ' '
                }
            );
        }
        println!();
    }
}

fn clean_map(map: &mut HashMap<(i32, i32), bool>) {
    let minx = *map.keys().map(|(x, _)| x).min().unwrap();
    let miny = *map.keys().map(|(_, y)| y).min().unwrap();
    let mut queue = VecDeque::new();
    queue.push_back((minx, miny));
    let mut visited: HashSet<(i32, i32)> = HashSet::new();
    while let Some((x, y)) = queue.pop_front() {
        if visited.contains(&(x, y)) {
            continue;
        }
        visited.insert((x, y));
        if !map.get(&(x, y)).unwrap_or(&false) {
            continue;
        }
        map.insert((x, y), false);
        for dx in -1..2 {
            for dy in -1..2 {
                let x = x + dx;
                let y = y + dy;
                if map.contains_key(&(x, y)) && !visited.contains(&(x, y)) {
                    queue.push_back((x, y));
                }
            }
        }
    }
}

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut map = HashMap::new();
    let mut algorithm = String::new();
    reader.read_line(&mut algorithm).unwrap();
    {
        let mut empty = String::new();
        reader.read_line(&mut empty).unwrap();
    }
    let mut width = 0;
    let mut height = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        if width == 0 {
            width = line.len();
        }
        for (x, c) in line.chars().enumerate() {
            map.insert((x as i32, height as i32), c == '#');
        }
        height += 1;
    }
    let mut output = HashMap::new();
    let mut two = 0;
    for iteration in 0..50 {
        //print_map(&map);
        println!("{}: {}", iteration, map.values().filter(|c| **c).count());
        let minx = *map
            .iter()
            .filter_map(|((x, _), v)| if *v { Some(x) } else { None })
            .min()
            .unwrap()
            - 4;
        let maxx = *map
            .iter()
            .filter_map(|((x, _), v)| if *v { Some(x) } else { None })
            .max()
            .unwrap()
            + 4;
        let miny = *map
            .iter()
            .filter_map(|((_, y), v)| if *v { Some(y) } else { None })
            .min()
            .unwrap()
            - 4;
        let maxy = *map
            .iter()
            .filter_map(|((_, y), v)| if *v { Some(y) } else { None })
            .max()
            .unwrap()
            + 4;
        for y in miny..maxy + 1 {
            for x in minx..maxx + 1 {
                let mut v: Vec<bool> = vec![];
                for dy in -1..2 {
                    for dx in -1..2 {
                        v.push(*map.get(&(x + dx, y + dy)).unwrap_or(&false));
                    }
                }
                let s = v
                    .iter()
                    .map(|i| if *i { '1' } else { '0' })
                    .collect::<String>();
                let index = usize::from_str_radix(&s, 2).unwrap();
                output.insert((x, y), algorithm.chars().nth(index).unwrap() == '#');
            }
        }
        map = output;
        output = HashMap::new();
        if iteration == 1 {
            let mut temp = map.clone();
            clean_map(&mut temp);
            two = temp.values().filter(|c| **c).count();
            println!("two {}", two);
        }
        if (iteration % 2) == 1 {
            clean_map(&mut map);
        }
    }
    print_map(&map);
    (two, map.values().filter(|c| **c).count())
}

fn main() {
    println!("{:?}", run("input/20-example.txt"));
    println!("{:?}", run("input/20.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_20() {
        let (first, second) = super::run("input/20-example.txt");
        assert_eq!(first, 35);
        assert_eq!(second, 3351);
    }

    #[test]
    fn test_input_20() {
        let (first, second) = super::run("input/20.txt");
        assert_eq!(first, 5359);
        assert_eq!(second, 12333);
    }
}
