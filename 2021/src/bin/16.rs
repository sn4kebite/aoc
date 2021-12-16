use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn parse_packet<'a>(it: &mut impl Iterator<Item = &'a u8>, level: usize) -> (usize, usize, usize) {
    //let prefix = "  ".repeat(level);
    let version = (it.next().unwrap() << 2) | (it.next().unwrap() << 1) | it.next().unwrap();
    //println!("{}version {}", prefix, version);
    let type_id = (it.next().unwrap() << 2) | (it.next().unwrap() << 1) | it.next().unwrap();
    //println!("{}type id {}", prefix, type_id);
    let mut size = 6;
    let mut v_sum: usize = version as usize;
    let mut subs = vec![];
    match type_id {
        4 => {
            let mut more = 1;
            let mut value = 0_usize;
            while more != 0 {
                more = *it.next().unwrap();
                //println!("{}more {}", prefix, more);
                for _ in 0..4 {
                    value = (value << 1) | *it.next().unwrap() as usize;
                }
                size += 5;
            }
            subs.push(value);
            //println!("{}value {}", prefix, value);
        }
        _ => {
            let length_type = *it.next().unwrap();
            size += 1;
            let mut value = 0usize;
            for _ in 0..15 - (4 * length_type) {
                value = (value << 1) | *it.next().unwrap() as usize;
                size += 1;
            }
            if length_type == 0 {
                //println!("{}total {}", prefix, value);
                let mut i = 0;
                while i < value {
                    let (size, sum, pval) = parse_packet(it, level + 1);
                    i += size;
                    //println!("{}size {}, remaining {}/{}", prefix, size, value - i, value);
                    v_sum += sum;
                    subs.push(pval);
                }
                size += value;
            } else {
                for _ in 0..value {
                    let (s, sum, pval) = parse_packet(it, level + 1);
                    v_sum += sum;
                    size += s;
                    subs.push(pval);
                }
                //println!("{}num_subs {}", prefix, value);
            }
        }
    }
    let value: usize = match type_id {
        0 => subs.iter().sum(),
        1 => subs.iter().product(),
        2 => *subs.iter().min().unwrap(),
        3 => *subs.iter().max().unwrap(),
        4 => *subs.first().unwrap(),
        5 => (subs[0] > subs[1]) as usize,
        6 => (subs[0] < subs[1]) as usize,
        7 => (subs[0] == subs[1]) as usize,
        _ => 0,
    };
    //println!("{}packet end size={} sum={} value={}", prefix, size, v_sum, value);
    (size, v_sum, value)
}

fn run_string(buf: &str) -> (usize, usize) {
    let v: Vec<u8> = buf
        .chars()
        .map(|c| {
            let c = c.to_digit(16).unwrap() as u8;
            [c >> 3, (c >> 2) & 1, (c >> 1) & 1, c & 1]
        })
        .flatten()
        .collect();
    let mut it = v.iter();
    let (_, sum, value) = parse_packet(&mut it, 0);
    (sum, value)
}

fn run(filename: &str) -> (usize, usize) {
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    reader.read_line(&mut buf).expect("line");
    run_string(buf.trim())
}

fn main() {
    // part 1
    println!("{:?}", run_string("8A004A801A8002F478"));
    println!("{:?}", run_string("620080001611562C8802118E34"));
    println!("{:?}", run_string("C0015000016115A2E0802F182340"));
    println!("{:?}", run_string("A0016C880162017C3686B18A3D4780"));

    // part 1
    println!("{:?}", run_string("C200B40A82"));
    println!("{:?}", run_string("04005AC33890"));
    println!("{:?}", run_string("880086C3E88112"));
    println!("{:?}", run_string("CE00C43D881120"));
    println!("{:?}", run_string("D8005AC2A8F0"));
    println!("{:?}", run_string("F600BC2D8F"));
    println!("{:?}", run_string("9C005AC2F8F0"));
    println!("{:?}", run_string("9C0141080250320F1802104A08"));

    println!("{:?}", run("input/16.txt"));
}

#[cfg(test)]
mod tests {
    fn get_first(s: &str) -> usize {
        super::run_string(s).0
    }

    fn get_second(s: &str) -> usize {
        super::run_string(s).1
    }

    #[test]
    fn test_part1_examples() {
        assert_eq!(get_first("8A004A801A8002F478"), 16);
        assert_eq!(get_first("620080001611562C8802118E34"), 12);
        assert_eq!(get_first("C0015000016115A2E0802F182340"), 23);
        assert_eq!(get_first("A0016C880162017C3686B18A3D4780"), 31);
    }

    #[test]
    fn test_part2_examples() {
        assert_eq!(get_second("C200B40A82"), 3);
        assert_eq!(get_second("04005AC33890"), 54);
        assert_eq!(get_second("880086C3E88112"), 7);
        assert_eq!(get_second("CE00C43D881120"), 9);
        assert_eq!(get_second("D8005AC2A8F0"), 1);
        assert_eq!(get_second("F600BC2D8F"), 0);
        assert_eq!(get_second("9C005AC2F8F0"), 0);
        assert_eq!(get_second("9C0141080250320F1802104A08"), 1);
    }

    #[test]
    fn test_input_16() {
        let (first, second) = super::run("input/16.txt");
        assert_eq!(first, 986);
        assert_eq!(second, 18234816469452);
    }
}
