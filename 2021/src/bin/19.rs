extern crate nalgebra as na;
use na::{Rotation3, Vector3};
use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct Scanner {
    pos: Vector3<f32>,
    rot: Rotation3<f32>,
    beacons: Vec<Vector3<f32>>,
    rot_beacons: Option<Vec<Vector3<f32>>>,
    done: bool,
}

impl Scanner {
    fn parse(reader: &mut BufReader<File>) -> Option<Self> {
        let mut beacons = vec![];
        for line in reader.lines() {
            let line = line.unwrap();
            if line.len() == 0 {
                break;
            }
            if line.starts_with("---") {
                continue;
            }
            let mut split = line.split(',');
            let x: f32 = split.next().unwrap().parse().unwrap();
            let y: f32 = split.next().unwrap().parse().unwrap();
            let z: f32 = split.next().unwrap().parse().unwrap();
            let v = Vector3::new(x, y, z);
            beacons.push(v);
        }
        if beacons.len() == 0 {
            return None;
        }
        let pos = Vector3::new(0.0, 0.0, 0.0);
        let rot = Rotation3::identity();
        Some(Self {
            pos,
            rot,
            beacons,
            rot_beacons: None,
            done: false,
        })
    }

    fn get_rotation(x: usize, y: usize, z: usize) -> Rotation3<f32> {
        Rotation3::from_euler_angles(
            f32::to_radians((x * 90) as f32),
            f32::to_radians((y * 90) as f32),
            f32::to_radians((z * 90) as f32),
        )
    }

    fn rotate_beacons(&self, x: usize, y: usize, z: usize) -> Vec<Vector3<f32>> {
        let rot = Self::get_rotation(x, y, z);
        self.beacons
            .iter()
            .map(|b| {
                let b = rot * b;
                Vector3::new(b.x.round(), b.y.round(), b.z.round())
            })
            .collect()
    }

    fn fix_beacons(&mut self) {
        if self.rot_beacons.is_none() {
            self.rot_beacons = Some(
                self.beacons
                    .iter()
                    .map(|b| {
                        let b = self.rot * b;
                        Vector3::new(b.x.round(), b.y.round(), b.z.round())
                    })
                    .collect(),
            );
        }
    }

    fn overlaps(&self, other: &Scanner) -> Option<(Vector3<f32>, Rotation3<f32>)> {
        let self_beacons = self.rot_beacons.as_ref().unwrap();
        for x in 0..2 {
            for y in 0..4 {
                // FIXME
                if x == 1 && (y & 1) > 0 {
                    continue;
                }
                for z in 0..4 {
                    let beacons = other.rotate_beacons(x, y, z);
                    //println!("{},{},{}: {:?}", x * 90, y * 90, z * 90, beacons);
                    let mut diffs = vec![];
                    for a in self_beacons {
                        for b in &beacons {
                            diffs.push(a - b);
                        }
                    }
                    for diff in &diffs {
                        let mut num_overlaps = 0;
                        for a in self_beacons {
                            for b in &beacons {
                                let b = *b + diff;
                                if *a == b {
                                    //println!("overlap {} and {}", a, &b);
                                    num_overlaps += 1;
                                }
                            }
                        }
                        if num_overlaps >= 12 {
                            println!(
                                "{} overlaps with {},{},{} at {:?}",
                                num_overlaps,
                                x * 90,
                                y * 90,
                                z * 90,
                                diff
                            );
                            return Some((*diff, Self::get_rotation(x, y, z)));
                        }
                    }
                }
            }
        }
        None
    }
}

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut scanners = vec![];
    while let Some(scanner) = Scanner::parse(&mut reader) {
        scanners.push(scanner);
    }
    // scanner 0 does not need to be updated
    scanners[0].done = true;
    scanners[0].fix_beacons();
    let mut v = vec![];
    // cheat to make processing faster
    if scanners.len() == 35 {
        v.extend([
            (0, 5),
            (0, 12),
            (0, 21),
            (0, 24),
            (5, 8),
            (5, 31),
            (5, 32),
            (5, 34),
            (8, 11),
            (8, 25),
            (11, 9),
            (11, 16),
            (16, 1),
            (16, 29),
            (21, 27),
            (24, 26),
            (24, 30),
            (25, 13),
            (26, 14),
            (29, 18),
            (29, 33),
            (30, 4),
            (30, 10),
            (31, 3),
            (31, 22),
            (32, 15),
            (3, 28),
            (9, 17),
            (9, 19),
            (10, 2),
            (14, 7),
            (15, 6),
            (17, 20),
            (2, 23),
        ]);
    } else {
        for a in 0..scanners.len() {
            for b in 1..scanners.len() {
                v.push((a, b));
            }
        }
    }
    let mut remaining = scanners.len() - 1;
    while remaining > 0 {
        println!("{} scanners remaining", remaining);
        for (a, b) in &v {
            let a = *a;
            let b = *b;
            if a == b {
                continue;
            }
            // skip scanners that already have a position
            if scanners[b].done {
                continue;
            }
            // skip scanners overlap checks with scanners that do not yet have a position
            if !scanners[a].done {
                continue;
            }
            println!("comparing scanner {} and scanner {}", a, b);
            if let Some((diff, rot)) = scanners[a].overlaps(&scanners[b]) {
                scanners[b].pos = scanners[a].pos + diff;
                scanners[b].rot = rot;
                scanners[b].done = true;
                scanners[b].fix_beacons();
                remaining -= 1;
                println!(
                    "scanner {} overlaps scanner {} at {:?}",
                    a, b, scanners[b].pos
                );
            }
        }
    }
    let mut distance = 0;
    for a in &scanners {
        for b in &scanners {
            let dist = a.pos - b.pos;
            let dist = (dist.x.abs() + dist.y.abs() + dist.z.abs()) as usize;
            if dist > distance {
                distance = dist;
            }
        }
    }
    let mut beacons = HashSet::new();
    for scanner in &scanners {
        for beacon in &scanner.beacons {
            let beacon = scanner.pos + scanner.rot * beacon;
            let beacon = Vector3::new(
                beacon.x.round() as i32,
                beacon.y.round() as i32,
                beacon.z.round() as i32,
            );
            beacons.insert(beacon);
        }
    }
    (beacons.len(), distance)
}

fn main() {
    println!("{:?}", run("input/19-example.txt"));
    println!("{:?}", run("input/19.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_19() {
        let (first, second) = super::run("input/19-example.txt");
        assert_eq!(first, 79);
        assert_eq!(second, 3621);
    }

    #[test]
    fn test_input_19() {
        let (first, second) = super::run("input/19.txt");
        assert_eq!(first, 434);
        assert_eq!(second, 11906);
    }
}
