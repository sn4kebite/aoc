//use std::env;
use std::io;
use std::iter::FromIterator;

fn main() {
    //let width: usize = env::args().nth(1).unwrap().parse().expect("Failed to parse width");
    //let height: usize = env::args().nth(2).unwrap().parse().expect("Failed to parse height");
    let (width, height) = (25, 6);
    let mut min_zeroes = std::usize::MAX;
    let mut result = 0;
    let mut data = String::new();
    io::stdin().read_line(&mut data).expect("Failed to read input");
    let mut layer = 0;
    data = data.trim().to_string();
    let chars = data.as_bytes();
    let mut image = Vec::new();
    image.resize(width*height, ' ');
    while layer*width*height < data.len() {
        let mut current_zeroes = 0;
        let mut ones = 0;
        let mut twos = 0;
        for i in 0..width*height {
            let pos = layer*width*height + i;
            match chars[pos] as char {
                '0' => {
                    current_zeroes += 1;
                    if image[i] == ' ' {
                        image[i] = '.';
                    }
                },
                '1' => {
                    ones += 1;
                    if image[i] == ' ' {
                        image[i] = '#';
                    }
                },
                '2' => twos += 1,
                val => println!("Unepxected value {}", val),
            }
        }
        if current_zeroes < min_zeroes {
            min_zeroes = current_zeroes;
            result = ones * twos;
        }
        layer += 1;
    }
    println!("Result: {}", result);
    for line in image.chunks(width) {
        let line = String::from_iter(line);
        println!("{}", line);
    }
}
