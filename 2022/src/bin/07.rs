use std::cell::RefCell;
use std::collections::VecDeque;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::rc::Rc;

struct Directory {
    pub parent: Option<Rc<RefCell<Directory>>>,
    pub path: String,
    pub subdirs: Vec<Rc<RefCell<Directory>>>,
    pub files: Vec<Rc<RefCell<FileEntry>>>,
}

impl Directory {
    pub fn root() -> Self {
        Directory {
            parent: None,
            path: String::from("/"),
            subdirs: vec![],
            files: vec![],
        }
    }

    pub fn cd(&mut self, name: &str) -> Rc<RefCell<Self>> {
        if name == ".." {
            return self
                .parent
                .as_ref()
                .expect("Cannot traverse up from root directory")
                .clone();
        }
        for d in &self.subdirs {
            if d.borrow().path == name {
                return d.clone();
            }
        }
        let new = Rc::new(RefCell::new(Directory {
            parent: None,
            path: name.to_owned(),
            subdirs: vec![],
            files: vec![],
        }));
        self.subdirs.push(new.clone());
        new
    }

    pub fn get_file(&mut self, name: &str, size: usize) -> Rc<RefCell<FileEntry>> {
        let f = Rc::new(RefCell::new(FileEntry {
            _name: name.to_owned(),
            size,
        }));
        self.files.push(f.clone());
        f
    }

    pub fn get_recursive_size(&self) -> usize {
        let mut size = 0;
        for f in &self.files {
            let f = f.borrow();
            size += f.size;
        }
        for d in &self.subdirs {
            size += d.borrow().get_recursive_size();
        }
        size
    }
}

struct FileEntry {
    pub _name: String,
    pub size: usize,
}

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let root = Rc::new(RefCell::new(Directory::root()));
    let mut current = root.clone();
    for line in reader.lines() {
        let line: String = line.unwrap();
        if line.starts_with("$ cd") {
            let name = &line[5..];
            if name == "/" {
                current = root.clone();
            } else {
                let new = current.borrow_mut().cd(name);
                // new directory
                if !Rc::ptr_eq(&new, &root) && new.borrow().parent.is_none() {
                    new.borrow_mut().parent = Some(current.clone());
                }
                current = new;
            }
        } else if line.starts_with("$ ls") {
            // nop
        } else {
            let (typ, name) = line.split_once(' ').unwrap();
            if typ == "dir" {
                current.borrow_mut().cd(name);
            } else {
                let size: usize = typ.parse().unwrap();
                current.borrow_mut().get_file(name, size);
            }
        }
    }
    let total_disk = 70000000;
    let need_disk = 30000000;
    let used_disk = root.borrow().get_recursive_size();
    let mut total_size = 0;
    let mut smallest_size = used_disk;
    let mut queue = VecDeque::new();
    queue.push_back(root.clone());
    while !queue.is_empty() {
        let current = queue.pop_front().expect("Queue wasn't empty");
        let current = current.borrow();
        let size = current.get_recursive_size();
        if size <= 100000 {
            total_size += size;
        }
        for d in &current.subdirs {
            queue.push_back(d.clone());
        }
    }
    queue.push_back(root.clone());
    while !queue.is_empty() {
        let current = queue.pop_front().expect("Queue wasn't empty");
        let current = current.borrow();
        let size = current.get_recursive_size();
        if total_disk - used_disk + size >= need_disk && size < smallest_size {
            smallest_size = size;
        }
        for d in &current.subdirs {
            queue.push_back(d.clone());
        }
    }
    (total_size, smallest_size)
}

fn main() {
    println!("{:?}", run("input/07-example.txt"));
    println!("{:?}", run("input/07.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_07() {
        let (first, second) = super::run("input/07-example.txt");
        assert_eq!(first, 95437);
        assert_eq!(second, 24933642);
    }

    #[test]
    fn test_07() {
        let (first, second) = super::run("input/07.txt");
        assert_eq!(first, 2061777);
        assert_eq!(second, 4473403);
    }
}
