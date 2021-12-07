use std::collections::HashMap;
use std::io;
use std::io::BufRead;

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
        true
    }
}

fn main() {
    let mut fields: HashMap<String, String> = HashMap::new();
    let mut passport: Passport;
    let mut valid = 0;
    let mut valid_strict = 0;
    for line in io::stdin().lock().lines() {
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
        fields = HashMap::new();
        if passport.is_valid() {
            valid += 1;
        }
        if passport.is_valid_strict() {
            valid_strict += 1;
        }
    }
    println!("valid: {}", valid);
    println!("strict: {}", valid_strict);
}
