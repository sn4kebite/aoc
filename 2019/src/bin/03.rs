use std::fmt;
use std::io;
use std::iter::FromIterator;
use std::ops::AddAssign;
use std::ops::Sub;

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn from_string(input: char) -> Result<Direction, ()> {
        match input {
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            _ => Err(()),
        }
    }
}

struct Vector {
    direction: Direction,
    length: i32,
}

impl Vector {
    fn from_string(input: &str) -> Vector {
        Vector { direction: Direction::from_string(input.chars().nth(0).unwrap()).unwrap(), length: input[1..].parse().unwrap() }
    }
}

#[derive(Copy, Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn from_vector(v: &Vector) -> Vec2 {
        match v.direction {
            Direction::Left => Vec2 {x: -v.length, y: 0},
            Direction::Right => Vec2 {x: v.length, y: 0},
            Direction::Up => Vec2 {x: 0, y: v.length},
            Direction::Down => Vec2 {x: 0, y: -v.length},
        }
    }
    fn length(&self) -> i32 {
        self.x.abs() + self.y.abs()
    }
}

impl fmt::Display for Vec2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, other: Self) {
        *self = Self { x: self.x + other.x, y: self.y + other.y };
    }
}

impl Sub for Vec2 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self { x: self.x - other.x, y: self.y - other.y }
    }
}

struct WireVector {
    start: Vec2,
    end: Vec2,
    last_length: i32,
}

fn is_between(n: i32, a: i32, b: i32) -> bool {
    if a < b {
        a < n && n < b
    } else {
        a > n && n > b
    }
}

impl WireVector {
    fn intersects(&self, other: &Self) -> Result<Vec2, ()> {
        let self_vertical = self.start.x == self.end.x;
        let other_vertical = other.start.x == other.end.x;
        if (self_vertical && other_vertical) || !(self_vertical || other_vertical) {
            return Err(());
        }
        if self_vertical {
            if is_between(self.start.x, other.start.x, other.end.x) && is_between(other.start.y, self.start.y, self.end.y) {
                Ok(Vec2{x: self.start.x, y: self.start.y + (other.start.y - self.start.y)})
            } else {
                Err(())
            }
        } else {
            if is_between(self.start.y, other.start.y, other.end.y) && is_between(other.start.x, self.start.x, self.end.x) {
                Ok(Vec2{x: self.start.x + (other.start.x - self.start.x), y: self.start.y})
            } else {
                Err(())
            }
        }
    }
}

impl fmt::Display for WireVector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.start, self.end)
    }
}

fn read_wire() -> Vec<WireVector> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)
        .expect("Failed to read line");
    let mut pos = Vec2 {x: 0, y: 0};
    let mut length = 0;
    Vec::<WireVector>::from_iter(line.trim().split(",")
        .map(|x| {
            let v = Vector::from_string(x);
            let cur = pos;
            pos += Vec2::from_vector(&v);
            let l = length;
            length += (pos - cur).length();
            WireVector {start: cur, end: pos, last_length: l}
        })
    )
}

fn main() {
    let wire1: Vec<_> = read_wire();
    let wire2: Vec<_> = read_wire();

    let mut distance = std::i32::MAX;
    let mut last_steps = std::i32::MAX;
    for v1 in wire1 {
        for v2 in &wire2 {
            match v1.intersects(v2) {
                Ok(x) => {
                    //println!("Intersect at {}", x);
                    let d = x.length();
                    if d < distance {
                        distance = d;
                    }
                    let s = v1.last_length + (x - v1.start).length()
                        + v2.last_length + (x - v2.start).length();
                    if s < last_steps {
                        last_steps = s;
                    }
                },
                Err(_) => (),
            }
        }
    }
    println!("Shortest distance: {}", distance);
    println!("Shortest steps: {}", last_steps);
}
