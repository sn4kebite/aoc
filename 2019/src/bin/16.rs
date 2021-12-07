use std::io;
use std::iter::FromIterator;

fn main() {
    let base_pattern = [0, 1, 0, -1];
    let mut input: Vec<i32> = {
        let mut line = String::new();
        io::stdin().read_line(&mut line)
            .expect("Failed to read line");
        Vec::from_iter(line.trim().chars().map(|e| e as i32 - 48))
    };
    /*input.reserve(input.len()*9999);
    {
        let len = input.len();
        for _i in 1..10000 {
            //input.concat(&input[0..len]);
            input = [&input[0..input.len()], &input[0..len]].concat();
        }
    }*/
    for i in 0..input.len() {
        print!("{}", input[i]);
    }
    println!("");
    println!("input has {} elements", input.len());
    //let mut phases: Vec<i32> = vec![];
    //phases.resize(input.len(), 1);
    //let mut phase = 1;
    //println!("{:?}", input);
    //println!("{:?}", phases);
    for phase in 0..100 {
        println!("phase {}", phase);
        let mut new_input = vec![];
        new_input.resize(input.len(), 0);
        for i in 0..input.len() {
            let mut output = 0;
            for _j in 0..i {
                print!("           ");
            }
            for j in i..input.len() {
                let phase_index = ((j + 1) / (i + 1)) % base_pattern.len();
                //println!("input={} i={} j={} phase_index={} pattern={}", input[j], i, j, phase_index, base_pattern[phase_index]);
                //if phase_index == 0 || phase_index == 2 {
                //    continue;
                //}
                print!("{}*{:2} ({}) + ", input[j], base_pattern[phase_index], phase_index);
                output += input[j] * base_pattern[phase_index];
                //phases[i] += 1;
            }
            //println!("");
            new_input[i] = output.abs() % 10;
            //print!("{:1}", new_input[i]);
            println!("= {:1} ({})", new_input[i], output);
            //if i < 8 {
            //    print!("{}", new_input[i]);
            //}
        }
        //println!("");
        //phase += 1;
        //println!("");
        input = new_input;
        //println!("{:?}", input.join(" "));
        //println!("{:?}", phases);
    }
    //for i in 0..8 {
    for i in 0..input.len() {
        if i == 8 {
            print!(" ");
        }
        print!("{}", input[i]);
    }
    println!("");
    let mut offset = 0;
    for i in 0..7 {
        offset += 10_i32.pow(6-i) * input[i as usize];
    }
    println!("Offset: {}", offset);
    print!("Message: ");
    for i in offset..offset+8 {
        print!("{}", input[(i as usize) % input.len()]);
    }
    println!("");
}
