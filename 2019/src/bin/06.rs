use std::io;
use std::io::BufRead;
use std::iter::FromIterator;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;

struct Step {
    name: String,
    steps: u32,
}

fn main() {
    let mut map = HashMap::new();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let v = Vec::from_iter(line.trim().split(")"));
        map.insert(v[1].to_string(), v[0].to_string());
    }
    let mut orbits = 0;
    //let mut revmap: HashMap<_, Vec<_>> = HashMap::new();
    let mut revmap = HashMap::new();
    for (from, to) in &map {
        // part 2
        revmap.entry(from.to_string())
            .or_insert_with(Vec::new)
            .push(to.to_string());
        revmap.entry(to.to_string())
            .or_insert_with(Vec::new)
            .push(from.to_string());
        if to != "COM" {
            let mut name = to;
            loop {
                orbits += 1;
                name = match map.get(name) {
                    Some(n) => n,
                    None => break,
                }
            }
        } else {
            orbits += 1;
        }
    }
    println!("Orbits: {}", orbits);
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(Step{name: "YOU".to_string(), steps: 0});
    while queue.len() > 0 {
        let step = queue.pop_front().unwrap();
        if step.name == "SAN" {
            // Subtract two because we need to ignore first and final steps
            println!("Found {} after {} steps", step.name, step.steps-2);
            break;
        }
        if visited.contains(&step.name) {
            continue;
        }
        visited.insert(step.name.to_string());
        for item in &revmap[&step.name] {
            if !visited.contains(item) {
                queue.push_back(Step{name: item.to_string(), steps: step.steps + 1});
            }
        }
    }
}
