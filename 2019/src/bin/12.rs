use std::cmp::Ordering;
use std::cmp::PartialEq;
use std::io;
use std::io::BufRead;
use std::iter::FromIterator;

use regex::Regex;
use num_integer::lcm;

#[derive(Debug, Clone, Copy)]
struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    fn len(&self) -> usize {
        (self.x.abs() + self.y.abs() + self.z.abs()) as usize
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[derive(Debug, Clone, Copy)]
struct Moon {
    pos: Vec3,
    vel: Vec3,
}

impl Moon {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self {pos: Vec3 {x: x, y: y, z: z}, vel: Vec3 {x: 0, y: 0, z: 0}}
    }
    fn energy(&self) -> usize {
        self.pos.len() * self.vel.len()
    }
    fn get_x(&self) -> (i32, i32) {
        (self.pos.x, self.vel.x)
    }
    fn get_y(&self) -> (i32, i32) {
        (self.pos.y, self.vel.y)
    }
    fn get_z(&self) -> (i32, i32) {
        (self.pos.z, self.vel.z)
    }
}

impl PartialEq for Moon {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.vel == other.vel
    }
}

fn step(moons: &mut Vec<Moon>) {
    for i in 0..moons.len() {
        let (start_moons, mid_moons) = moons.split_at_mut(i);
        let (moon, end_moons) = mid_moons.split_at_mut(1);
        let mut moon = &mut moon[0];
        //println!("{:?}", moon);
        for m in start_moons.iter_mut().chain(end_moons) {
            match m.pos.x.cmp(&moon.pos.x) {
                Ordering::Less => moon.vel.x -= 1,
                Ordering::Greater => moon.vel.x += 1,
                _ => (),
            }
            match m.pos.y.cmp(&moon.pos.y) {
                Ordering::Less => moon.vel.y -= 1,
                Ordering::Greater => moon.vel.y += 1,
                _ => (),
            }
            match m.pos.z.cmp(&moon.pos.z) {
                Ordering::Less => moon.vel.z -= 1,
                Ordering::Greater => moon.vel.z += 1,
                _ => (),
            }
        }
    }
    for moon in moons.iter_mut() {
        moon.pos.x += moon.vel.x;
        moon.pos.y += moon.vel.y;
        moon.pos.z += moon.vel.z;
    }
    //for moon in moons {
    //    println!("  {:?}", moon);
    //}
}

fn main() {
    let re = Regex::new(r"<x=(-?\d+), y=(-?\d+), z=(-?\d+)>").unwrap();
    let mut moons: Vec<Moon> = Vec::from_iter(io::stdin().lock().lines().map(
        |line| {
            let line = line.unwrap();
            let matches = re.captures(line.as_str()).unwrap();
            Moon::new(matches[1].parse().unwrap(), matches[2].parse().unwrap(), matches[3].parse().unwrap())
        }
    ));
    let initial = moons.clone();
    let mut iterations = 0;
    for _i in 0..1000 {
        //println!("step {}:", i);
        step(&mut moons);
        //println!("");
        iterations += 1;
    }
    let mut energy = 0;
    for moon in moons.iter() {
        //println!("{:?} has energy {}", moon, moon.energy());
        energy += moon.energy();
    }
    let mut moon_periods = (0, 0, 0);
    println!("Energy: {}", energy);
    moons = initial.clone();
    iterations = 0;
    'outer: loop {
        step(&mut moons);
        iterations += 1;
        if iterations < 2 {
            continue;
        }
        if moon_periods.0 == 0 {
            let mut matched = true;
            for i in 0..moons.len() {
                if moons[i].get_x() != initial[i].get_x() {
                    matched = false;
                }
            }
            if matched {
                moon_periods.0 = iterations;
            }
        }
        if moon_periods.1 == 0 {
            let mut matched = true;
            for i in 0..moons.len() {
                if moons[i].get_y() != initial[i].get_y() {
                    matched = false;
                }
            }
            if matched {
                moon_periods.1 = iterations;
            }
        }
        if moon_periods.2 == 0 {
            let mut matched = true;
            for i in 0..moons.len() {
                if moons[i].get_z() != initial[i].get_z() {
                    matched = false;
                }
            }
            if matched {
                moon_periods.2 = iterations;
            }
        }
        if moon_periods.0 > 0 && moon_periods.1 > 0 && moon_periods.2 > 0 {
            break;
        }
    }
    let periods = vec![moon_periods.0 as u128, moon_periods.1 as u128, moon_periods.2 as u128];
    println!("{:?}", periods);
    let mut period: u128 = 1;
    for p in periods.iter() {
        period = lcm(period, *p);
    }
    println!("Periods: {}", period);
}
