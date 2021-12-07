use std::io;
use std::io::BufRead;
use std::collections::HashMap;

fn run(mut state: &mut HashMap<String, u64>, map: &HashMap<String, (u32, Vec<(u32, String)>)>, target: &str, count: u64) -> u64 {
    //println!("making {} {}", count, target);
    let mut ore = 0;
    let mut count = count;
    for (produce, dest) in map.get(target) {
        let produce_count = (count as f64 / *produce as f64).ceil() as u64;
        //println!("  {}, {:?}", produce, dest);
        /*let mut need_total = HashMap::new();
        for (req, next) in dest {
            let mut needed = count*req/produce;
            if needed % req > 0 {
                needed += req - needed % req;
            }
            //need_total.insert(next, needed);
            need_total.entry(next)
                .and_modify(|e| *e += needed)
                .or_insert(needed);
        }*/
        //println!("need total for {}: {:?}", target, need_total);
        let mut crafted = true;
        /*while crafted {
            crafted = false;
            for (name, total) in need_total.iter() {
                let current = *state.entry(name.to_string())
                    .or_insert(0);
                print!("has {} {}", current, name);
                if current < *total {
                    crafted = true;
                    println!(", missing {}", total - current);
                    ore += run(&mut state, &map, name, total - current);
                } else {
                    println!("");
                }
            }
        }*/
        while crafted {
            crafted = false;
            for (req, next) in dest {
                //let needed = (count*produce+req)/req;
                /*let mut needed = count*req/produce;
                if needed % req > 0 {
                    needed += req - needed % req;
                }*/
                let needed = *req as u64 * produce_count;
                let current = *state.entry(next.to_string())
                    .or_insert(0);
                //print!("has {} {}", current, next);
                if current < needed {
                    //println!(", missing {}", needed - current);
                    ore += run(&mut state, &map, next, needed - current);
                    crafted = true;
                } else {
                    //println!("");
                }
            }
            //print!("(modified {} to {:?}, req={}) ", next, state.get(next).unwrap(), req);
        }
        for (req, next) in dest {
            /*let mut needed = count*req/produce;
            if needed % req > 0 {
                needed += req - needed % req;
            }*/
            let needed = *req as u64 * produce_count;
            state.entry(next.to_string())
                .and_modify(|e| { *e -= needed });
            let new = *state.get(&next.to_string()).unwrap();
            if new < 0 {
                panic!("got {} {}!", new, next);
            }
        }
        /*let mut p = *produce;
        while p < count {
            p += produce;
        }
        count = p;*/
        count = *produce as u64 * produce_count;
        break;
    }
    //println!("added {} {}", count, target);
    state.entry(target.to_string())
        .and_modify(|e| { *e += count })
        .or_insert(count);
    if target == "ORE" {
        ore += count as u64;
    }
    ore
}

fn main() {
    let mut map = HashMap::new();
    for line in io::stdin().lock().lines() {
        let line = line.unwrap();
        let (from, to) = {
            let t: Vec<_> = line.split(" => ").collect();
            assert_eq!(t.len(), 2);
            (t[0].to_string(), t[1].to_string())
        };
        //println!("{} from:", to);
        let mut v = Vec::new();
        for item in from.split(", ") {
            //println!("  {}", item);
            let (n, chemical) = {
                let t: Vec<_> = item.split(' ').collect();
                assert_eq!(t.len(), 2);
                (t[0].to_string(), t[1].to_string())
            };
            let n: u32 = n.parse().unwrap();
            v.push((n, chemical));
        }
        let (n, to) = {
            let t: Vec<_> = to.split(' ').collect();
            assert_eq!(t.len(), 2);
            (t[0].to_string(), t[1].to_string())
        };
        let n: u32 = n.parse().unwrap();
        map.insert(to, (n, v));
    }
    let mut state = HashMap::new();
    //let ore = map.get("ORE").unwrap().0;
    let mut ore = run(&mut state, &map, "FUEL", 1);
    //let ore = run(&mut state, &map, "A", 11);
    //println!("Used {} ore", ore - map.get("ORE").unwrap().0);
    println!("{} ore", ore);
    let target: u64 = 1000000000000;
    let mut low = 1000000;
    let mut high = 10000000;
    let mut last;
    while ore != target {
        state.clear();
        last = low + (high - low) / 2;
        ore = run(&mut state, &map, "FUEL", last);
        println!("ore={} low={} high={} last={}", ore, low, high, last);
        if ore < target {
            low = last;
            if (high as i64 - low as i64).abs() < 2 {
                high += 2;
            }
        } else if ore > target {
            high = last;
            if (high as i64 - low as i64).abs() < 2 {
                low -= 2;
            }
        }
    }
    println!("{} ore", ore);
}
