use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Clone)]
struct Cube {
    fx: i64,
    tx: i64,
    fy: i64,
    ty: i64,
    fz: i64,
    tz: i64,
}

impl Cube {
    fn parse(s: &str) -> Self {
        let cuboids = s.split(',');
        let coords: Vec<(i64, i64)> = cuboids
            .map(|c| {
                let mut split = c.split('=');
                split.next().unwrap();
                let mut split = split.next().unwrap().split("..");
                (
                    split.next().unwrap().parse::<i64>().unwrap(),
                    split.next().unwrap().parse::<i64>().unwrap(),
                )
            })
            .collect();
        let (fx, tx) = coords[0];
        let (fy, ty) = coords[1];
        let (fz, tz) = coords[2];
        Self {
            fx,
            tx,
            fy,
            ty,
            fz,
            tz,
        }
    }

    fn volume(&self) -> i64 {
        (self.tx - self.fx + 1) * (self.ty - self.fy + 1) * (self.tz - self.fz + 1)
    }

    fn intersect(&self, other: &Self) -> Option<Self> {
        if !self.overlaps(&other) {
            return None;
        }
        let fx = self.fx.max(other.fx);
        let tx = self.tx.min(other.tx);
        let fy = self.fy.max(other.fy);
        let ty = self.ty.min(other.ty);
        let fz = self.fz.max(other.fz);
        let tz = self.tz.min(other.tz);
        Some(Self {
            fx,
            tx,
            fy,
            ty,
            fz,
            tz,
        })
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.fx <= other.tx
            && other.fx <= self.tx
            && self.fy <= other.ty
            && other.fy <= self.ty
            && self.fz <= other.tz
            && other.fz <= self.tz
    }

    fn split(&self, other: &Cube) -> Vec<Self> {
        let mut splits = vec![];
        let intersect = match self.intersect(&other) {
            Some(intersect) => intersect,
            None => return splits,
        };
        // split lower x part
        if self.fx < intersect.fx {
            splits.push(Self {
                fx: self.fx,
                tx: intersect.fx - 1,
                fy: self.fy,
                ty: self.ty,
                fz: self.fz,
                tz: self.tz,
            });
        }
        // split upper x part
        if self.tx > intersect.tx {
            splits.push(Self {
                fx: intersect.tx + 1,
                tx: self.tx,
                fy: self.fy,
                ty: self.ty,
                fz: self.fz,
                tz: self.tz,
            });
        }
        // split lower y part, constrain to intersect x
        if self.fy < intersect.fy {
            splits.push(Self {
                fx: intersect.fx,
                tx: intersect.tx,
                fy: self.fy,
                ty: intersect.fy - 1,
                fz: self.fz,
                tz: self.tz,
            });
        }
        // split upper y part, constrain to intersect x
        if self.ty > intersect.ty {
            splits.push(Self {
                fx: intersect.fx,
                tx: intersect.tx,
                fy: intersect.ty + 1,
                ty: self.ty,
                fz: self.fz,
                tz: self.tz,
            });
        }
        // split lower z part
        if self.fz < intersect.fz {
            splits.push(Self {
                fx: intersect.fx,
                tx: intersect.tx,
                fy: intersect.fy,
                ty: intersect.ty,
                fz: self.fz,
                tz: intersect.fz - 1,
            });
        }
        // split upper z part
        if self.tz > intersect.tz {
            splits.push(Self {
                fx: intersect.fx,
                tx: intersect.tx,
                fy: intersect.fy,
                ty: intersect.ty,
                fz: intersect.tz + 1,
                tz: self.tz,
            });
        }
        splits
    }
}

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut cubes = HashMap::new();
    let mut steps = vec![];
    for line in reader.lines() {
        let line = line.unwrap();
        let (state, line) = line.split_once(' ').unwrap();
        let cube = Cube::parse(&line);
        let state = state == "on";
        steps.push((cube.clone(), state));
        if cube.fx > 50
            || cube.tx < -50
            || cube.fy > 50
            || cube.ty < -50
            || cube.fz > 50
            || cube.tz < -50
        {
            continue;
        }
        let fx = cube.fx.min(50).max(-50);
        let tx = cube.tx.min(50).max(-50);
        let fy = cube.fy.min(50).max(-50);
        let ty = cube.ty.min(50).max(-50);
        let fz = cube.fz.min(50).max(-50);
        let tz = cube.tz.min(50).max(-50);
        for x in fx..tx + 1 {
            for y in fy..ty + 1 {
                for z in fz..tz + 1 {
                    cubes.insert((x, y, z), state);
                }
            }
        }
    }
    let initialization = cubes.values().filter(|c| **c).count();
    let mut cubes = vec![];
    for (cube, state) in steps {
        let mut new_cubes = vec![];
        for old in &cubes {
            if cube.overlaps(&old) {
                new_cubes.extend(old.split(&cube));
            } else {
                new_cubes.push(old.clone());
            }
        }
        if state {
            new_cubes.push(cube.clone());
        }
        cubes = new_cubes;
    }
    (
        initialization,
        cubes.iter().map(|cube| cube.volume()).sum::<i64>() as usize,
    )
}

fn main() {
    println!("{:?}", run("input/22-example1.txt"));
    println!("{:?}", run("input/22-example2.txt"));
    println!("{:?}", run("input/22-example3.txt"));
    println!("{:?}", run("input/22.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example1_22() {
        let (first, _) = super::run("input/22-example1.txt");
        assert_eq!(first, 39);
    }

    #[test]
    fn test_example2_22() {
        let (first, _) = super::run("input/22-example2.txt");
        assert_eq!(first, 590784);
    }

    #[test]
    fn test_example3_22() {
        let (first, second) = super::run("input/22-example3.txt");
        assert_eq!(first, 474140);
        assert_eq!(second, 2758514936282235);
    }

    #[test]
    fn test_input_22() {
        let (first, second) = super::run("input/22.txt");
        assert_eq!(first, 577205);
        assert_eq!(second, 1197308251666843);
    }
}
