use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Vector2 {
    x: i32,
    y: i32,
}

impl Vector2 {
    //type IterTo = VectorIterTo;

    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    fn iter_to(&self, other: &Self) -> VectorIterTo {
        let xd = if self.x == other.x {
            0
        } else {
            (other.x - self.x) / (other.x - self.x).abs()
        };
        let yd = if self.y == other.y {
            0
        } else {
            (other.y - self.y) / (other.y - self.y).abs()
        };
        VectorIterTo {
            v: Vector2 {
                x: self.x,
                y: self.y,
            },
            to: Vector2 {
                x: other.x + xd,
                y: other.y + yd,
            },
            xd,
            yd,
        }
    }
}

impl FromStr for Vector2 {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let v: Vec<i32> = s.split(',').map(|d| d.parse().unwrap()).collect();
        Ok(Self::new(v[0], v[1]))
    }
}

#[derive(Debug)]
struct VectorIterTo {
    v: Vector2,
    to: Vector2,
    xd: i32,
    yd: i32,
}

impl Iterator for VectorIterTo {
    type Item = Vector2;

    fn next(&mut self) -> Option<Self::Item> {
        //println!("{:?}", self);
        if self.v.x == self.to.x && self.v.y == self.to.y {
            return None;
        }
        let current = self.v;
        self.v.x += self.xd;
        self.v.y += self.yd;
        Some(current)
    }
}

#[derive(Debug)]
struct Vent {
    from: Vector2,
    to: Vector2,
}

impl Vent {
    fn new(from: Vector2, to: Vector2) -> Self {
        Self { from, to }
    }
}

impl FromStr for Vent {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut v: Vec<Vector2> = s.split(" -> ").map(|s| s.parse().unwrap()).collect();
        let b = v.pop().unwrap();
        let a = v.pop().unwrap();
        Ok(Self::new(a, b))
    }
}

fn calc_overlaps(vents: &Vec<Vent>, skip_diagonals: bool) -> usize {
    let mut overlaps: HashMap<Vector2, usize> = HashMap::new();
    let mut num_overlaps = 0;
    for vent in vents {
        if skip_diagonals && vent.from.x != vent.to.x && vent.from.y != vent.to.y {
            continue;
        }
        for v in vent.from.iter_to(&vent.to) {
            let e = overlaps.entry(v).or_insert(0);
            if *e == 1 {
                //println!("overlap at {:?}", v);
                num_overlaps += 1;
            }
            *e += 1;
            //println!("{:?}", v);
        }
    }
    println!("Number of overlaps: {}", num_overlaps);
    num_overlaps
}

pub fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let vents: Vec<Vent> = reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            let line = line.trim();
            line.parse().unwrap()
        })
        .collect();
    //println!("{:?}", vents);
    (calc_overlaps(&vents, true), calc_overlaps(&vents, false))
}

mod day05 {
    pub use super::run;
}

fn main() {
    run("input/05-example.txt");
    run("input/05.txt");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_05() {
        let (first, second) = super::run("input/05-example.txt");
        assert_eq!(first, 5);
        assert_eq!(second, 12);
    }

    #[test]
    fn test_input_05() {
        let (first, second) = super::run("input/05.txt");
        assert_eq!(first, 8622);
        assert_eq!(second, 22037);
    }
}
