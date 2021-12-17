use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn find_hit(fx: i32, tx: i32, fy: i32, ty: i32, vx: i32, vy: i32) -> Option<i32> {
    let mut x = 0;
    let mut y = 0;
    let mut vx = vx;
    let mut vy = vy;
    let mut hy = i32::min_value();
    while x <= tx && y >= fy {
        if x >= fx && y <= ty {
            return Some(hy);
        }
        x += vx;
        y += vy;
        if y > hy {
            hy = y;
        }
        if vx > 0 {
            vx -= 1;
        }
        vy -= 1;
    }
    None
}

fn run(filename: &str) -> (i32, i32) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("line");
    let buf = buf.trim();
    let (_, buf) = buf.split_once(": ").unwrap();
    let (x, y) = buf.split_once(", ").unwrap();
    let (_, x) = x.split_once('=').unwrap();
    let (_, y) = y.split_once('=').unwrap();
    let (fx, tx) = x.split_once("..").unwrap();
    let (fy, ty) = y.split_once("..").unwrap();
    let fx: i32 = fx.parse().unwrap();
    let tx: i32 = tx.parse().unwrap();
    let fy: i32 = fy.parse().unwrap();
    let ty: i32 = ty.parse().unwrap();
    let mut hy = i32::min_value();
    let mut hits = 0;
    for vx in 0..500 {
        for vy in -500..500 {
            if let Some(i) = find_hit(fx, tx, fy, ty, vx, vy) {
                hits += 1;
                if i > hy {
                    hy = i;
                }
            }
        }
    }
    (hy, hits)
}

fn main() {
    println!("{:?}", run("input/17-example.txt"));
    println!("{:?}", run("input/17.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_17() {
        let (first, second) = super::run("input/17-example.txt");
        assert_eq!(first, 45);
        assert_eq!(second, 112);
    }

    #[test]
    fn test_input_17() {
        let (first, second) = super::run("input/17.txt");
        assert_eq!(first, 7381);
        assert_eq!(second, 3019);
    }
}
