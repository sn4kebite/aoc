use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn run_insertion(filename: &str, steps: usize) -> usize {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut template = String::new();
    reader.read_line(&mut template).expect("template");
    template = template.trim().to_string();
    {
        let mut buf = String::new();
        reader.read_line(&mut buf).expect("empty line");
    }
    let mut rules: HashMap<String, char> = HashMap::new();
    reader.lines().for_each(|line| {
        let line = line.expect("line");
        let (from, to) = line.split_once(" -> ").unwrap();
        let from = String::from(from);
        let to = to.chars().nth(0).expect("to");
        rules.insert(from, to);
    });
    let mut rule_mutations: HashMap<String, (String, String)> = HashMap::new();
    for (from, to) in &rules {
        rule_mutations.insert(
            from.clone(),
            (
                String::from(
                    vec![&from.chars().nth(0).unwrap(), to]
                        .into_iter()
                        .collect::<String>(),
                ),
                String::from(
                    vec![to, &from.chars().nth(1).unwrap()]
                        .into_iter()
                        .collect::<String>(),
                ),
            ),
        );
    }
    let mut rule_count: HashMap<String, usize> = HashMap::new();
    let mut element_count: HashMap<char, usize> = HashMap::new();
    for c in template.chars() {
        *element_count.entry(c).or_insert(0) += 1;
    }
    for i in 0..template.len() - 1 {
        for (from, _) in &rules {
            let from = from.clone();
            if template[i..i + 2] == *from {
                *rule_count.entry(from).or_insert(0) += 1;
            }
        }
    }
    for _ in 0..steps {
        let mut add_rules = vec![];
        for (from, (a, b)) in &rule_mutations {
            let e = rule_count.entry(from.to_string()).or_insert(0);
            if *e > 0 {
                add_rules.push((a.to_string(), *e));
                add_rules.push((b.to_string(), *e));
                *element_count.entry(b.chars().nth(0).unwrap()).or_insert(0) += *e;
                *e = 0;
            }
        }
        for (rule, count) in add_rules {
            *rule_count.entry(rule).or_insert(0) += count;
        }
    }
    element_count.values().max().unwrap() - element_count.values().min().unwrap()
}

fn run(filename: &str) -> (usize, usize) {
    (run_insertion(filename, 10), run_insertion(filename, 40))
}

fn main() {
    println!("{:?}", run("input/14-example.txt"));
    println!("{:?}", run("input/14.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_14() {
        let (first, second) = super::run("input/14-example.txt");
        assert_eq!(first, 1588);
        assert_eq!(second, 2188189693529);
    }

    #[test]
    fn test_input_14() {
        let (first, second) = super::run("input/14.txt");
        assert_eq!(first, 3342);
        assert_eq!(second, 3776553567525);
    }
}
