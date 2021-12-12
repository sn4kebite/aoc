use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[allow(non_camel_case_types)]
#[derive(Debug, Clone)]
enum Instruction {
    acc(i32),
    jmp(i32),
    nop(i32),
}

impl Instruction {
    fn parse(s: &str) -> Self {
        let (name, operand) = {
            let mut split = s.split_whitespace();
            (
                split.next().unwrap(),
                split.next().unwrap().parse::<i32>().unwrap(),
            )
        };
        match name {
            "acc" => Instruction::acc(operand),
            "jmp" => Instruction::jmp(operand),
            "nop" => Instruction::nop(operand),
            _ => panic!("invalid instruction {}", name),
        }
    }
}

#[derive(Clone)]
struct VM {
    instructions: Vec<Instruction>,
    acc: i32,
    pc: usize,
}

impl VM {
    fn from_reader(reader: &mut impl BufRead) -> Self {
        let mut instructions = vec![];
        for line in reader.lines() {
            let line = line.unwrap();
            instructions.push(Instruction::parse(line.as_str()));
        }
        Self {
            instructions,
            acc: 0,
            pc: 0,
        }
    }

    pub fn step(&mut self) {
        let ins = match self.instructions.get(self.pc) {
            Some(ins) => ins,
            None => {
                self.pc = 0;
                return;
            }
        };
        match ins {
            Instruction::acc(op) => self.acc += op,
            Instruction::jmp(op) => {
                // TODO signed adds from experimental
                self.pc = ((self.pc as i32) + op) as usize;
                return;
            }
            Instruction::nop(_) => (),
        }
        self.pc += 1;
    }

    pub fn get_acc(&self) -> i32 {
        self.acc
    }

    pub fn get_pc(&self) -> usize {
        self.pc
    }

    pub fn len(&self) -> usize {
        self.instructions.len()
    }

    pub fn mutate(&mut self, index: usize) {
        let ins = self.instructions.get_mut(index).expect("instruction");
        match ins {
            Instruction::jmp(op) => *ins = Instruction::nop(*op),
            Instruction::nop(op) => *ins = Instruction::jmp(*op),
            _ => (),
        }
    }

    pub fn reset(&mut self) {
        self.pc = 0;
        self.acc = 0;
    }
}

fn run(filename: &str) -> (i32, i32) {
    let mut vm = VM::from_reader({
        let file = File::open(filename).unwrap();
        &mut BufReader::new(file)
    });
    let mut visited = HashSet::new();
    while !visited.contains(&vm.get_pc()) {
        visited.insert(vm.get_pc());
        vm.step();
    }
    let acc = vm.get_acc();
    vm.reset();
    let mut mutate_acc = 0;
    'mutate: for index in 0..vm.len() {
        let mut vm = vm.clone();
        vm.mutate(index);
        visited.clear();
        while !visited.contains(&vm.get_pc()) {
            visited.insert(vm.get_pc());
            vm.step();
            if vm.get_pc() == vm.len() {
                mutate_acc = vm.get_acc();
                break 'mutate;
            }
        }
    }
    (acc, mutate_acc)
}

fn main() {
    println!("{:?}", run("input/08-example.txt"));
    println!("{:?}", run("input/08.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_08() {
        let (first, second) = super::run("input/08-example.txt");
        assert_eq!(first, 5);
        assert_eq!(second, 8);
    }

    #[test]
    fn test_input_08() {
        let (first, second) = super::run("input/08.txt");
        assert_eq!(first, 1801);
        assert_eq!(second, 2060);
    }
}
