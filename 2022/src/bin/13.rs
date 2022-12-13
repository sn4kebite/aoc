use std::cmp::Ordering;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Clone)]
enum Value {
    Int(usize),
    List(Vec<Value>),
}

impl Value {
    pub fn parse_rec(s: &str) -> (Self, usize) {
        if s.chars().all(|c| c.is_digit(10)) {
            let v = Value::Int(s.parse().unwrap());
            return (v, s.len());
        }
        //println!("parse_rec '{}'", s);
        let mut values = vec![];
        let start = 1;
        let mut current = start;
        while current < s.len() {
            //println!("at {} {}", current, s.chars().nth(current).unwrap());
            if s.chars().nth(current).unwrap() == '[' {
                //println!("nested list");
                let (sub, new) = Self::parse_rec(&s[current..]);
                current += new;
                //println!("nested list values {:?}", sub);
                values.push(sub);
                continue;
            } else if s.chars().nth(current).unwrap() == ']' {
                current += 1;
                break;
            }
            let n = s
                .chars()
                .enumerate()
                .position(|(i, c)| i > current && (c == ',' || c == '[' || c == ']'))
                .unwrap_or(0);
            if n == 0 {
                break;
            }
            let c = s.chars().nth(n).unwrap();
            if s.chars().nth(current).unwrap() == ',' {
                current += 1;
                continue;
            }
            let v = &s[current..n];
            //println!("value {:?}", v);
            //println!("next {}", n);
            let (sub, _) = Self::parse_rec(v);
            values.push(sub);
            current = n + 1;
            if c == ']' {
                break;
            }
        }
        //println!("ret {:?}", values);
        (Value::List(values), current)
    }

    pub fn parse(s: &str) -> Self {
        let (v, _) = Self::parse_rec(s);
        v
    }

    fn to_usize(&self) -> Option<usize> {
        if let Value::Int(v) = self {
            return Some(*v);
        }
        None
    }

    fn to_vec(&self) -> Option<&Vec<Value>> {
        if let Value::List(v) = self {
            return Some(v);
        }
        None
    }

    fn validate(&self, rhs: &Value) -> Option<bool> {
        let self_int = self.to_usize().is_some();
        let rhs_int = rhs.to_usize().is_some();
        if self_int && rhs_int {
            let self_int = self.to_usize().unwrap();
            let rhs_int = rhs.to_usize().unwrap();
            if self_int < rhs_int {
                return Some(true);
            }
            if self_int > rhs_int {
                return Some(false);
            }
        }
        if !self_int && !rhs_int {
            let self_vec = self.to_vec().unwrap();
            let rhs_vec = rhs.to_vec().unwrap();
            for i in 0..self_vec.len().max(rhs_vec.len()) {
                if i >= self_vec.len() {
                    return Some(true);
                }
                if i >= rhs_vec.len() {
                    return Some(false);
                }
                let sub = self_vec[i].validate(&rhs_vec[i]);
                if sub.is_some() {
                    return sub;
                }
            }
        }
        if self_int && !rhs_int {
            let left = Value::Int(self.to_usize().unwrap());
            let left = Value::List(vec![left]);
            return left.validate(&rhs);
        }
        if !self_int && rhs_int {
            let rhs = Value::Int(rhs.to_usize().unwrap());
            let rhs = Value::List(vec![rhs]);
            return self.validate(&rhs);
        }
        None
    }
}

struct Pair {
    left: Value,
    right: Value,
}

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut pairs = vec![];
    let mut packets = vec![];
    loop {
        let mut line = String::new();
        let r = reader.read_line(&mut line);
        if r.is_err() {
            break;
        }
        if line.trim().len() == 0 {
            reader.read_line(&mut line).unwrap();
        }
        if line.trim().len() == 0 {
            break;
        }
        let left = Value::parse(line.trim());
        line.clear();
        reader.read_line(&mut line).unwrap();
        let right = Value::parse(line.trim());
        packets.push(left.clone());
        packets.push(right.clone());
        pairs.push(Pair { left, right });
    }
    let mut sum = 0;
    for (i, p) in pairs.iter().enumerate() {
        let valid = p.left.validate(&p.right);
        let valid = valid.unwrap_or(false);
        if valid {
            sum += i + 1;
        }
    }
    let k1 = Value::parse("[[2]]");
    let k2 = Value::parse("[[6]]");
    packets.push(k1.clone());
    packets.push(k2.clone());
    packets.sort_by(|a, b| {
        if a.validate(&b).unwrap_or(false) {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    });
    let a = packets.iter().position(|i| i == &k1).unwrap() + 1;
    let b = packets.iter().position(|i| i == &k2).unwrap() + 1;
    (sum, a * b)
}

fn main() {
    println!("{:?}", run("input/13-example.txt"));
    println!("{:?}", run("input/13.txt"));
}

#[cfg(test)]
mod tests {
    use super::Value;

    #[test]
    fn test_13_parse() {
        assert_eq!(Value::parse("[]"), Value::List(vec![]));
        assert_eq!(Value::parse("[1]"), Value::List(vec![Value::Int(1)]));
        assert_eq!(
            Value::parse("[1,2,3]"),
            Value::List(vec![Value::Int(1), Value::Int(2), Value::Int(3)])
        );
        assert_eq!(
            Value::parse("[[1]]"),
            Value::List(vec![Value::List(vec![Value::Int(1)])])
        );
        assert_eq!(
            Value::parse("[[1,2]]"),
            Value::List(vec![Value::List(vec![Value::Int(1), Value::Int(2)])])
        );
        assert_eq!(
            Value::parse("[1,[2]]"),
            Value::List(vec![Value::Int(1), Value::List(vec![Value::Int(2)])])
        );
        assert_eq!(
            Value::parse("[[1],2]"),
            Value::List(vec![Value::List(vec![Value::Int(1)]), Value::Int(2)])
        );
        assert_eq!(
            Value::parse("[[1],[2]]"),
            Value::List(vec![
                Value::List(vec![Value::Int(1)]),
                Value::List(vec![Value::Int(2)])
            ])
        );
        assert_eq!(
            Value::parse("[1,[2,[3,4]],5]"),
            Value::List(vec![
                Value::Int(1),
                Value::List(vec![
                    Value::Int(2),
                    Value::List(vec![Value::Int(3), Value::Int(4)]),
                ]),
                Value::Int(5)
            ])
        );
    }

    #[test]
    fn test_example_13() {
        let (first, second) = super::run("input/13-example.txt");
        assert_eq!(first, 13);
        assert_eq!(second, 140);
    }

    #[test]
    fn test_13() {
        let (first, second) = super::run("input/13.txt");
        assert_eq!(first, 6101);
        assert_eq!(second, 21909);
    }
}
