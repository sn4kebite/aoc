use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn parse_line(line: &str) -> ((isize, isize), (isize, isize)) {
    let (sensor, beacon) = line.split_once(':').unwrap();

    let (_, sensor) = sensor.split_once(" at").unwrap();
    let sensor = sensor.split_once(", ").unwrap();
    let sensor = (
        sensor.0.split_once('=').unwrap().1.parse().unwrap(),
        sensor.1.split_once('=').unwrap().1.parse().unwrap(),
    );

    let (_, beacon) = beacon.split_once(" at").unwrap();
    let beacon = beacon.split_once(", ").unwrap();
    let beacon = (
        beacon.0.split_once('=').unwrap().1.parse().unwrap(),
        beacon.1.split_once('=').unwrap().1.parse().unwrap(),
    );
    (sensor, beacon)
}

fn run(filename: &str, check_y: isize, max_coord: isize) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut beacons = HashSet::new();
    let mut sensor_distances = HashSet::new();
    let mut min_x = isize::MAX;
    let mut max_x = isize::MIN;
    for line in reader.lines() {
        let line = line.unwrap();
        let (sensor, beacon) = parse_line(line.as_str());
        beacons.insert(beacon);
        let distance = (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs();
        sensor_distances.insert((sensor, distance));
        min_x = min_x.min(sensor.0 - distance);
        max_x = max_x.max(sensor.0 + distance);
    }
    let invalid_positions = (min_x..max_x + 1)
        .filter(|x| sensor_distances.iter().any(|(sensor, sensor_distance)| {
                let pos_distance = (x - sensor.0).abs() + (check_y - sensor.1).abs();
                pos_distance <= *sensor_distance && pos_distance > 0
            })
        )
        .count()
        // ignore coordinates that matches existing beacons
        - beacons.iter().filter(|b| b.1 == check_y).count();

    let mut tuning_freq = 0;
    'tf: for (sensor, distance) in &sensor_distances {
        for i in 0..distance + 2 {
            let x1 = sensor.0 - i;
            let x2 = sensor.0 + i;
            let y1 = sensor.1 + distance + 1 - i;
            let y2 = sensor.1 - distance - 1 + i;
            for (x, y) in [(x1, y1), (x1, y2), (x2, y2), (x2, y1)] {
                if x < 0 || x > max_coord || y < 0 || y > max_coord {
                    continue;
                }
                if !sensor_distances.iter().any(|(sensor, sensor_distance)| {
                    let distance = (x - sensor.0).abs() + (y - sensor.1).abs();
                    distance <= *sensor_distance
                }) {
                    tuning_freq = (x * 4000000 + y) as usize;
                    break 'tf;
                }
            }
        }
    }

    (invalid_positions, tuning_freq)
}

fn main() {
    println!("{:?}", run("input/15-example.txt", 10, 20));
    println!("{:?}", run("input/15.txt", 2000000, 4000000));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_15() {
        let (first, second) = super::run("input/15-example.txt", 10, 20);
        assert_eq!(first, 26);
        assert_eq!(second, 56000011);
    }

    #[test]
    fn test_15() {
        let (first, second) = super::run("input/15.txt", 2000000, 4000000);
        assert_eq!(first, 5176944);
        assert_eq!(second, 13350458933732);
    }
}
