use std::collections::HashMap;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

const REQUIRED_FIELDS: &'static [&'static str] = &["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

struct Passport {
    fields: HashMap<String, String>,
}

impl Passport {
    fn new(fields: HashMap<String, String>) -> Self {
        Self { fields }
    }

    fn is_valid(&self) -> bool {
        for field in REQUIRED_FIELDS {
            // FIXME
            if !self.fields.contains_key(&field.to_string()) {
                return false;
            }
        }
        true
    }

    fn is_valid_strict(&self) -> bool {
        if !self.is_valid() {
            return false;
        }
        let byr: usize = self.fields.get("byr").unwrap().parse().unwrap();
        if byr < 1920 || byr > 2002 {
            return false;
        }
        let iyr: usize = self.fields.get("iyr").unwrap().parse().unwrap();
        if iyr < 2010 || iyr > 2020 {
            return false;
        }
        let eyr: usize = self.fields.get("eyr").unwrap().parse().unwrap();
        if eyr < 2020 || eyr > 2030 {
            return false;
        }
        let hgt = self.fields.get("hgt").unwrap();
        match &hgt[hgt.len() - 2..] {
            "cm" => {
                let hgt = hgt[0..hgt.len() - 2].parse::<usize>().unwrap();
                if hgt < 150 || hgt > 193 {
                    return false;
                }
            }
            "in" => {
                let hgt = hgt[0..hgt.len() - 2].parse::<usize>().unwrap();
                if hgt < 59 || hgt > 76 {
                    return false;
                }
            }
            _ => return false,
        }
        let hcl = self.fields.get("hcl").unwrap();
        if hcl.len() != 7 || hcl.chars().nth(0).unwrap() != '#' {
            return false;
        }
        for c in hcl[1..].chars() {
            match c.to_lowercase() {
                _ if c.is_numeric() => (),
                _ if c >= 'a' && c <= 'f' => (),
                _ => return false,
            }
        }
        let ecl = self.fields.get("ecl").unwrap();
        match ecl.as_str() {
            "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => (),
            _ => return false,
        }
        let pid = self.fields.get("pid").unwrap();
        if pid.len() != 9 || !pid.chars().all(|c| c.is_numeric()) {
            return false;
        }
        true
    }
}

fn run(filename: &str) -> (usize, usize) {
    let mut fields: HashMap<String, String> = HashMap::new();
    let mut passport: Passport;
    let mut valid = 0;
    let mut valid_strict = 0;
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let line = line.trim();
        if line.len() == 0 {
            if fields.len() > 0 {
                passport = Passport::new(fields);
                fields = HashMap::new();
                if passport.is_valid() {
                    valid += 1;
                }
                if passport.is_valid_strict() {
                    valid_strict += 1;
                }
            }
            continue;
        }
        line.split_whitespace().for_each(|f| {
            let fv: Vec<&str> = f.split(':').collect();
            let key = fv[0];
            let value = fv[1];
            fields.insert(key.to_string(), value.to_string());
        });
    }
    if fields.len() > 0 {
        passport = Passport::new(fields);
        if passport.is_valid() {
            valid += 1;
        }
        if passport.is_valid_strict() {
            valid_strict += 1;
        }
    }
    (valid, valid_strict)
}

fn main() {
    println!("{:?}", run("input/04-example1.txt"));
    println!("{:?}", run("input/04-example2.txt"));
    println!("{:?}", run("input/04-example3.txt"));
    println!("{:?}", run("input/04.txt"));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_04() {
        let (first, _) = super::run("input/04-example1.txt");
        assert_eq!(first, 2);
        let (_, second) = super::run("input/04-example2.txt");
        assert_eq!(second, 0);
        let (_, second) = super::run("input/04-example3.txt");
        assert_eq!(second, 4);
    }

    #[test]
    fn test_input_04() {
        let (first, second) = super::run("input/04.txt");
        assert_eq!(first, 208);
        assert_eq!(second, 167);
    }
}
