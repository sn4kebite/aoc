use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Vector3 {
    x: usize,
    y: usize,
    z: usize,
}

impl Vector3 {
    pub fn parse(s: &str) -> Self {
        let mut split = s.splitn(3, ',');
        Self {
            x: split.next().unwrap().parse().unwrap(),
            y: split.next().unwrap().parse().unwrap(),
            z: split.next().unwrap().parse().unwrap(),
        }
    }

    pub fn distance(&self, other: &Vector3) -> usize {
        ((self.x as isize - other.x as isize).pow(2)
            + (self.y as isize - other.y as isize).pow(2)
            + (self.z as isize - other.z as isize).pow(2))
        .isqrt() as usize
    }
}

fn run(filename: &str, connections: usize) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let boxes: Vec<Vector3> = reader
        .lines()
        .map(|line| Vector3::parse(line.unwrap().as_str()))
        .collect();
    let mut distances = vec![];
    for (i, a) in boxes.iter().enumerate() {
        for (j, b) in boxes.iter().enumerate() {
            if i <= j {
                continue;
            }
            distances.push((i, j, a.distance(b)));
        }
    }
    distances.sort_by_key(|v| v.2);
    let mut circuits: Vec<Vec<usize>> = vec![];
    let mut prod = 0;
    let mut last = (0, 0);
    let mut skipped = 0;
    for current_index in 0..distances.len() {
        if current_index == connections {
            let mut circuits = circuits.clone();
            circuits.sort_by_key(|c| c.len());
            circuits.reverse();
            prod = circuits.iter().take(3).map(|c| c.len()).product();
        }
        let (i, j, _) = distances[current_index];
        if let Some(ci) = circuits
            .iter()
            .position(|c| c.contains(&i) || c.contains(&j))
        {
            if circuits[ci].contains(&i) && circuits[ci].contains(&j) {
                // Break if we have two many skips in a row; we're probably done
                skipped += 1;
                // Randomly chosen threshold
                if skipped > connections * 2 {
                    break;
                }
            } else {
                // Get the box index to insert
                let bi = if circuits[ci].contains(&i) { j } else { i };
                // X coordinates of the last two boxes
                last = (boxes[i].x, boxes[j].x);
                if let Some(jj) = circuits.iter().position(|c| c.contains(&bi)) {
                    // Pop the existing circuit and merge it
                    let other = circuits.remove(jj);
                    circuits[if ci < jj { ci } else { ci - 1 }].extend(other);
                } else {
                    circuits[ci].push(bi);
                }
                skipped = 0;
            }
        } else {
            circuits.push(Vec::from([i, j]));
            skipped = 0;
        }
    }
    circuits.sort_by_key(|c| c.len());
    circuits.reverse();
    (prod, last.0 * last.1)
}

fn main() {
    println!("{:?}", run("input/08-example.txt", 10));
    println!("{:?}", run("input/08.txt", 1000));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_08() {
        let (first, second) = super::run("input/08-example.txt", 10);
        assert_eq!(first, 40);
        assert_eq!(second, 25272);
    }

    #[test]
    fn test_08() {
        let (first, second) = super::run("input/08.txt", 1000);
        assert_eq!(first, 83520);
        assert_eq!(second, 1131823407);
    }
}
