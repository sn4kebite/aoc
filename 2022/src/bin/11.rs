use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

/// Operation operand
#[derive(Debug)]
enum OpOp {
    Old,
    Imm(usize),
}

#[derive(Debug)]
struct Monkey {
    pub items: Vec<usize>,
    pub operator: fn(usize, usize) -> usize,
    pub operand: OpOp,
    pub divisible_by: usize,
    pub if_true: usize,
    pub if_false: usize,
    pub inspects: usize,
}

impl Monkey {
    pub fn parse(reader: &mut BufReader<File>) -> Option<Self> {
        let mut items = vec![];
        let mut operator: fn(usize, usize) -> usize = std::ops::Add::add;
        let mut operand = OpOp::Old;
        let mut divisible_by = 0;
        let mut if_true = 0;
        let mut if_false = 0;
        let mut has_monkey = false;
        for line in reader.lines() {
            let line = line.unwrap();
            let line: &str = line.trim();
            if line.len() == 0 {
                break;
            }
            if line.starts_with("Monkey ") {
                has_monkey = true;
            }
            if line.starts_with("Starting items:") {
                let (_, item_str) = line.split_once(": ").unwrap();
                items = item_str.split(", ").map(|v| v.parse().unwrap()).collect();
            }
            if line.starts_with("Operation:") {
                let mut split = line.split_whitespace().rev();
                operand = match split.next().unwrap() {
                    "old" => OpOp::Old,
                    v => OpOp::Imm(v.parse().unwrap()),
                };
                operator = match split.next().unwrap() {
                    "+" => std::ops::Add::add,
                    "*" => std::ops::Mul::mul,
                    v => panic!("Unknown operator {}", v),
                };
            }
            if line.starts_with("Test:") {
                divisible_by = line
                    .split_whitespace()
                    .rev()
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();
            }
            if line.starts_with("If true:") {
                if_true = line
                    .split_whitespace()
                    .rev()
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();
            }
            if line.starts_with("If false:") {
                if_false = line
                    .split_whitespace()
                    .rev()
                    .next()
                    .unwrap()
                    .parse()
                    .unwrap();
            }
        }
        if !has_monkey {
            return None;
        }
        Some(Self {
            items,
            operator,
            operand,
            divisible_by,
            if_true,
            if_false,
            inspects: 0,
        })
    }

    /// Takes an item and returns the next monkey
    pub fn item_to_monkey(&self, item: usize) -> usize {
        if item % self.divisible_by == 0 {
            self.if_true
        } else {
            self.if_false
        }
    }

    pub fn modify_item(&self, item: usize) -> usize {
        let v = match self.operand {
            OpOp::Old => item,
            OpOp::Imm(v) => v,
        };
        (self.operator)(item, v)
    }
}

fn _run(filename: &str, monkey_time: bool) -> usize {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut monkeys = vec![];
    while let Some(monkey) = Monkey::parse(&mut reader) {
        monkeys.push(monkey);
    }
    let f: usize = monkeys.iter().map(|m| m.divisible_by).product();
    let rounds = if monkey_time { 10000 } else { 20 };
    for _ in 0..rounds {
        // container for new items since we can't modify other monkeys inside the loop
        let mut monkey_mods: HashMap<usize, Vec<usize>> = HashMap::new();
        //println!("Round {}", round + 1);
        for (i, m) in monkeys.iter_mut().enumerate() {
            if let Entry::Occupied(o) = &mut monkey_mods.entry(i) {
                let items = o.get_mut();
                for item in items.iter() {
                    m.items.push(*item)
                }
                items.clear();
            }
            for item in &m.items {
                m.inspects += 1;
                let mut new_value = m.modify_item(*item);
                if monkey_time {
                    new_value %= f;
                } else {
                    new_value /= 3;
                }
                let new_i = m.item_to_monkey(new_value);
                //println!("  Item {} → {} to monkey {}", *item, new_value, new_i);
                monkey_mods.entry(new_i).or_default().push(new_value);
            }
            //println!("Monkey {} inspected {} items", i, m.items.len());
            m.items.clear();
        }
        for (mi, items) in &mut monkey_mods {
            for item in items.iter() {
                monkeys[*mi].items.push(*item)
            }
            items.clear();
        }
    }
    monkeys.sort_by_key(|m| -(m.inspects as isize));
    monkeys[0..2].iter().map(|m| m.inspects).product()
}

fn run(filename: &str) -> (usize, usize) {
    (_run(filename, false), _run(filename, true))
}

fn main() {
    println!("{:?}", run("input/11-example.txt"));
    println!("{:?}", run("input/11.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_11() {
        let (first, second) = super::run("input/11-example.txt");
        assert_eq!(first, 10605);
        assert_eq!(second, 2713310158);
    }

    #[test]
    fn test() {
        let (first, second) = super::run("input/11.txt");
        assert_eq!(first, 78960);
        assert_eq!(second, 14561971968);
    }
}
