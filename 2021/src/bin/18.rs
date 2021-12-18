use std::collections::VecDeque;
use std::fmt;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, Clone)]
enum Param {
    Number(usize),
    Pair(Box<Pair>),
}

impl fmt::Display for Param {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Param::Number(n) => write!(f, "{}", n),
            Param::Pair(p) => write!(f, "{}", p),
        }
    }
}

impl Param {
    fn magnitude(&self) -> usize {
        match self {
            Param::Number(n) => *n,
            Param::Pair(p) => p.magnitude(),
        }
    }
}

#[derive(Debug, Clone)]
struct Pair {
    left: Param,
    right: Param,
}

impl fmt::Display for Pair {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{},{}]", self.left, self.right)
    }
}

impl Pair {
    fn parse_param(buf: &str) -> (Param, usize) {
        let mut numbuf = String::new();
        for (i, c) in buf.chars().enumerate() {
            match c {
                '[' => {
                    let (param, pos) = Pair::parse_param(&buf[i + 1..]);
                    return (param, pos + i);
                }
                ']' | ',' => return (Param::Number(numbuf.parse().unwrap()), i),
                c if c.is_numeric() => numbuf.push(c),
                _ => panic!("invalid character {}", c),
            }
        }
        panic!("should not happen");
    }

    fn parse_pair(buf: &str) -> (Pair, usize) {
        let mut left: Option<Param> = None;
        let mut right: Option<Param> = None;
        let mut pos = 0;
        let mut it = buf.chars().enumerate();
        it.next().unwrap();
        while let Some((i, c)) = it.next() {
            pos += 1;
            match c {
                '[' => {
                    let (param, p) = Pair::parse_pair(&buf[i..]);
                    let param = Some(Param::Pair(Box::new(param)));
                    if left.is_none() {
                        left = param;
                    } else {
                        right = param;
                    }
                    for _ in 0..p {
                        it.next();
                    }
                    pos += p;
                }
                ',' => (),
                c if c.is_numeric() => {
                    let (param, p) = Self::parse_param(&buf[i..]);
                    if left.is_none() {
                        left = Some(param);
                    } else {
                        right = Some(param);
                    }
                    for _ in 0..p {
                        it.next();
                    }
                    pos += p;
                    if right.is_some() {
                        break;
                    }
                }
                ']' => break,
                _ => panic!("invalid character {}", c),
            }
        }
        let left = left.unwrap();
        let right = right.unwrap();
        (Self { left, right }, pos)
    }

    fn parse(buf: &str) -> Self {
        let (pair, _) = Self::parse_pair(&buf);
        pair
    }

    fn reduce_rec(
        &mut self,
        explode: bool,
        l: &mut Option<&mut Param>,
        r: &mut Option<&mut Param>,
        level: usize,
    ) -> bool {
        //println!("level {}: {}", level, self);
        match &mut self.left {
            Param::Number(n) => {
                let n = *n;
                if n > 9 && !explode {
                    let m = n % 2;
                    let left = Param::Number(n / 2);
                    let right = Param::Number(n / 2 + m);
                    self.left = Param::Pair(Box::new(Pair { left, right }));
                    return true;
                }
            }
            Param::Pair(p) => {
                if level >= 3
                    && matches!(p.left, Param::Number(_))
                    && matches!(p.right, Param::Number(_))
                {
                    //println!("exploding {:?} ({:?}, {:?}", p, l, r);
                    let a = match p.left {
                        Param::Number(n) => n,
                        _ => panic!(),
                    };
                    //println!("l before {:?}", l);
                    if let Some(ref mut b) = l.as_mut() {
                        if let Param::Number(n) = *b {
                            ***b = Param::Number(*n + a);
                        } else {
                            panic!();
                        }
                    }
                    //println!("l after {:?}", l);
                    let a = match p.right {
                        Param::Number(n) => n,
                        _ => panic!(),
                    };
                    let mut r = Some(&mut self.right);
                    while let Some(Param::Pair(a)) = r {
                        r = Some(&mut a.left);
                    }
                    if let Some(ref mut b) = r.as_mut() {
                        if let Param::Number(n) = *b {
                            //println!("right: {} → {}", *n, *n + a);
                            ***b = Param::Number(*n + a);
                        }
                    }
                    self.left = Param::Number(0);
                    return true;
                } else {
                    let mut r = Some(&mut self.right);
                    while let Some(Param::Pair(a)) = r {
                        r = Some(&mut a.left);
                    }
                    if p.reduce_rec(explode, l, &mut r, level + 1) {
                        return true;
                    }
                }
            }
        }
        match &mut self.right {
            Param::Number(n) => {
                let n = *n;
                if n > 9 && !explode {
                    let m = n % 2;
                    let left = Param::Number(n / 2);
                    let right = Param::Number(n / 2 + m);
                    self.right = Param::Pair(Box::new(Pair { left, right }));
                    return true;
                }
            }
            Param::Pair(p) => {
                if level >= 3
                    && matches!(p.left, Param::Number(_))
                    && matches!(p.right, Param::Number(_))
                {
                    //println!("exploding {:?} ({:?}, {:?}", p, l, r);
                    let a = match p.left {
                        Param::Number(n) => n,
                        _ => panic!(),
                    };
                    let mut l = Some(&mut self.left);
                    while let Some(Param::Pair(a)) = l {
                        l = Some(&mut a.right);
                    }
                    if let Some(ref mut b) = l.as_mut() {
                        if let Param::Number(n) = *b {
                            //println!("left: {} → {}", *n, *n + a);
                            ***b = Param::Number(*n + a);
                        } else {
                            panic!();
                        }
                    }
                    let a = match p.right {
                        Param::Number(n) => n,
                        _ => panic!(),
                    };
                    //println!("r before {:?}", r);
                    if let Some(ref mut b) = r.as_mut() {
                        if let Param::Number(n) = *b {
                            //println!("modify r");
                            ***b = Param::Number(*n + a);
                        }
                    }
                    //println!("r after {:?}", r);
                    self.right = Param::Number(0);
                    return true;
                } else {
                    let mut l = Some(&mut self.left);
                    while let Some(Param::Pair(a)) = l {
                        l = Some(&mut a.right);
                    }
                    if p.reduce_rec(explode, &mut l, r, level + 1) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn reduce(&mut self) {
        while self.reduce_rec(true, &mut None, &mut None, 0)
            || self.reduce_rec(false, &mut None, &mut None, 0)
        {}
    }

    fn magnitude(&self) -> usize {
        self.left.magnitude() * 3 + self.right.magnitude() * 2
    }
}

fn run_string(buf: &str) -> (usize, usize) {
    let mut pair = Pair::parse(&buf);
    println!("{:?}", pair);
    pair.reduce();
    (pair.magnitude(), 0)
}

fn parse_file(filename: &str) -> Vec<Pair> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    reader
        .lines()
        .map(|line| {
            let line = line.unwrap();
            Pair::parse(&line)
        })
        .collect()
}

fn run(filename: &str) -> (usize, usize) {
    /*let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut numbers = VecDeque::new();
    for line in reader.lines() {
        let line = line.unwrap();
        let mut pair = Pair::parse(&line);
        println!("parsed pair: {}", pair);
        numbers.push_back(pair);
    }*/
    let numbers = parse_file(&filename);
    let mut largest = 0;
    for i in 0..numbers.len() {
        for j in i + 1..numbers.len() {
            let a = &numbers[i];
            let b = &numbers[j];
            let mut pair = Pair {
                left: Param::Pair(Box::new(a.clone())),
                right: Param::Pair(Box::new(b.clone())),
            };
            pair.reduce();
            let m = pair.magnitude();
            if m > largest {
                largest = m;
            }
            pair = Pair {
                left: Param::Pair(Box::new(b.clone())),
                right: Param::Pair(Box::new(a.clone())),
            };
            pair.reduce();
            let m = pair.magnitude();
            if m > largest {
                largest = m;
            }
        }
    }
    let mut numbers = VecDeque::from(numbers);
    let mut pair = numbers.pop_front().unwrap();
    while let Some(n) = numbers.pop_front() {
        pair = Pair {
            left: Param::Pair(Box::new(pair.clone())),
            right: Param::Pair(Box::new(n)),
        };
        //println!("pair addition: {}", pair);
        pair.reduce();
        //println!("reduced: {}", pair);
    }
    (pair.magnitude(), largest)
}

fn main() {
    println!("{:?}", run_string("[[[[[9,8],1],2],3],4]"));
    println!("{:?}", run_string("[7,[6,[5,[4,[3,2]]]]]"));
    println!("{:?}", run_string("[[1,2],[[3,4],5]]"));
    println!("{:?}", run_string("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"));
    println!("{:?}", run_string("[[[[1,1],[2,2]],[3,3]],[4,4]]"));
    println!("{:?}", run_string("[[[[3,0],[5,3]],[4,4]],[5,5]]"));
    println!("{:?}", run_string("[[[[5,0],[7,4]],[5,5]],[6,6]]"));
    println!("{:?}", run_string("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]][[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"));
    println!("{:?}", run("input/18-example1.txt"));
    println!("{:?}", run("input/18-example2.txt"));
    println!("{:?}", run("input/18-example3.txt"));
    println!("{:?}", run("input/18.txt"));
}

#[cfg(test)]
mod tests {
    use super::{Pair, Param};

    fn get_first(s: &str) -> usize {
        super::run_string(s).0
    }

    #[test]
    fn test_magnitudes_18() {
        assert_eq!(get_first("[[1,2],[[3,4],5]]"), 143);
        assert_eq!(get_first("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"), 1384);
        assert_eq!(get_first("[[[[1,1],[2,2]],[3,3]],[4,4]]"), 445);
        assert_eq!(get_first("[[[[3,0],[5,3]],[4,4]],[5,5]]"), 791);
        assert_eq!(get_first("[[[[5,0],[7,4]],[5,5]],[6,6]]"), 1137);
        assert_eq!(get_first("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]][[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"), 3488);
    }

    fn test_addition(input: &[&str], expected: &str) {
        let mut pair = Pair::parse(input[0]);
        for n in &input[1..] {
            pair = Pair {
                left: Param::Pair(Box::new(pair.clone())),
                right: Param::Pair(Box::new(Pair::parse(n))),
            };
            pair.reduce();
        }
        assert_eq!(format!("{}", pair), expected);
    }

    #[test]
    fn test_addition_18() {
        test_addition(
            &["[1,1]", "[2,2]", "[3,3]", "[4,4]"],
            "[[[[1,1],[2,2]],[3,3]],[4,4]]",
        );
        test_addition(
            &["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]"],
            "[[[[3,0],[5,3]],[4,4]],[5,5]]",
        );
        test_addition(
            &["[1,1]", "[2,2]", "[3,3]", "[4,4]", "[5,5]", "[6,6]"],
            "[[[[5,0],[7,4]],[5,5]],[6,6]]",
        );
    }

    #[test]
    fn test_example_18() {
        let (first, second) = super::run("input/18-example3.txt");
        assert_eq!(first, 4140);
        assert_eq!(second, 3993);
    }

    #[test]
    fn test_input_18() {
        let (first, second) = super::run("input/18.txt");
        assert_eq!(first, 4008);
        assert_eq!(second, 4667);
    }
}
