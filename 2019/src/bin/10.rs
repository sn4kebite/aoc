use std::cmp;
use std::convert::TryInto;
use std::io;
use std::io::BufRead;
use std::iter::FromIterator;
use std::iter::Iterator;
use std::collections::BTreeMap;

fn is_visible(map: &Vec<Vec<char>>, fx: i32, fy: i32, tx: i32, ty: i32) -> bool {
    //println!("is_visible {},{} -> {},{}", fx, fy, tx, ty);
    let dx = tx - fx;
    let dy = ty - fy;
    let dist = cmp::max(dx.abs(), dy.abs());
    let angle = (dy as f64).atan2(dx as f64);
    let step = ((dx as f64).powf(2.0) + (dy as f64).powf(2.0)).sqrt() / dist as f64;
    //println!("d {},{} angle {} cos {} sin {} step {}", dx, dy, angle, angle.cos(), angle.sin(), step);
    //println!("dist {}", dist);
    for i in 0..dist {
        let f_x = fx as f64 + angle.cos() * i as f64 * step;
        let f_y = fy as f64 + angle.sin() * i as f64 * step;
        if (f_x.round() - f_x).abs() > 0.01 || (f_y.round() - f_y).abs() > 0.01 {
            //println!("Skipping because {} or {}", f_x.round() - f_x, f_y.round() - f_y);
            continue;
        }
        let x = f_x.round() as i32;
        let y = f_y.round() as i32;
        //println!("  {}: {},{} == {} ({},{})", i, x, y, map[y as usize][x as usize], f_x, f_y);
        if (x != fx || y != fy) && (x != tx || y != ty) && map[y as usize][x as usize] == '#' {
            //println!("  Blocked at {},{}", x, y);
            return false;
        }
    }
    return true
}

fn find_visible(map: &Vec<Vec<char>>, x: i32, y: i32) -> usize {
    let mut count = 0;
    for (ay, line) in map.iter().enumerate() {
        for (ax, &c) in line.iter().enumerate() {
            if c == '#' {
                if ax as i32 == x && ay as i32 == y {
                    //println!("Skipping because {},{} {},{}", ax, ay, x, y);
                    continue;
                }
                if is_visible(map, ax.try_into().unwrap(), ay.try_into().unwrap(), x, y) {
                    //println!("visible!");
                    count += 1;
                }
            }
        }
    }
    count
}

fn find_best(map: &Vec<Vec<char>>) -> (usize, usize, usize) {
    let (mut bx, mut by) = (0, 0);
    let mut last_visible = 0;
    for (y, line) in map.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == '#' {
                let visible = find_visible(&map, x.try_into().unwrap(), y.try_into().unwrap());
                //println!("Asteroid at {},{} sees {} asteroids", x, y, visible);
                if visible > last_visible {
                    last_visible = visible;
                    bx = x;
                    by = y;
                }
            }
        }
    }
    (bx, by, last_visible)
}

fn make_laser_map(map: &Vec<Vec<char>>, station: (usize, usize)) -> BTreeMap<i64, Vec<(usize, usize)>> {
    let mut angle_map: BTreeMap<i64, Vec<(usize, usize)>> = BTreeMap::new();
    for (y, line) in map.iter().enumerate() {
        for (x, &c) in line.iter().enumerate() {
            if c == '#' && (x != station.0 || y != station.1) {
                let mut asteroid_angle = ((station.1 as i32 - y as i32) as f64).atan2((x as i32 - station.0 as i32) as f64);
                asteroid_angle -= std::f64::consts::PI/2.0;
                //asteroid_angle += std::f64::consts::PI*2.0;
                asteroid_angle = std::f64::consts::PI*2.0 - asteroid_angle;
                if asteroid_angle < 0.0 {
                    asteroid_angle += std::f64::consts::PI*2.0;
                }
                if asteroid_angle >= std::f64::consts::PI*2.0 {
                    asteroid_angle -= std::f64::consts::PI*2.0;
                }
                //angle = get_angle(&mut angle_map, asteroid_angle);
                angle_map.entry((asteroid_angle * 1000.0) as i64)
                    .or_insert_with(Vec::new)
                    .push((x, y));
            }
        }
    }
    for (_angle, asteroids) in angle_map.iter_mut() {
        let distance = |ast: &(usize, usize)| {
            let dx = ast.0 as i32 - station.0 as i32;
            let dy = ast.1 as i32 - station.1 as i32;
            ((dx as f64).powf(2.0) + (dy as f64).powf(2.0)).sqrt()
        };
        asteroids.sort_by(|a, b| distance(b).partial_cmp(&distance(a)).unwrap());
    }
    angle_map
}

fn main() {
    let map: Vec<Vec<char>> = Vec::from_iter(io::stdin().lock().lines().map(
        |line| {
            Vec::from_iter(line.unwrap().chars())
        }
    ));
    let (bx, by, count) = find_best(&map);
    println!("Best: {},{} sees {} asteroids", bx, by, count);
    let mut count = 0;
    let mut laser_map = make_laser_map(&map, (bx, by));
    let mut found = true;
    while found {
        found = false;
        for (_angle, asteroids) in laser_map.iter_mut() {
            if !asteroids.is_empty() {
                found = true;
                let ast = asteroids.pop().unwrap();
                count += 1;
                //println!("{}: {},{}", *angle as f64 / 1000.0, ast.0, ast.1);
                //println!("{}: {},{}", *angle, ast.0, ast.1);
                if count == 200 {
                    //println!("200 {},{}", ast.0, ast.1);
                    println!("{}", ast.0*100 + ast.1);
                    return
                }
            }
        }
    }
}
