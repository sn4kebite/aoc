fn run_rec(
    turn: bool,
    p1_score: usize,
    p2_score: usize,
    p1_pos: usize,
    p2_pos: usize,
    roll: usize,
) -> (usize, usize) {
    let mut p1_pos = p1_pos;
    let mut p2_pos = p2_pos;
    let mut p1_score = p1_score;
    let mut p2_score = p2_score;
    if turn {
        p1_pos = (p1_pos + roll) % 10;
        p1_score += p1_pos + 1;
        if p1_score >= 21 {
            return (1, 0);
        }
    } else {
        p2_pos = (p2_pos + roll) % 10;
        p2_score += p2_pos + 1;
        if p2_score >= 21 {
            return (0, 1);
        }
    }
    let (mut p1_wins, mut p2_wins) = (0, 0);
    for (roll, count) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        let (p1, p2) = run_rec(!turn, p1_score, p2_score, p1_pos, p2_pos, roll);
        p1_wins += p1 * count;
        p2_wins += p2 * count;
    }
    (p1_wins, p2_wins)
}

fn run(p1_start: usize, p2_start: usize) -> (usize, usize) {
    let mut p1_pos = p1_start - 1;
    let mut p2_pos = p2_start - 1;
    let mut p1_score = 0;
    let mut p2_score = 0;
    let mut it = (1..101).cycle();
    let mut rolls = 0;
    loop {
        let v = it.next().unwrap() + it.next().unwrap() + it.next().unwrap();
        p1_pos = (p1_pos + v) % 10;
        p1_score += p1_pos + 1;
        rolls += 3;

        if p1_score >= 1000 {
            break;
        }

        let v = it.next().unwrap() + it.next().unwrap() + it.next().unwrap();
        p2_pos = (p2_pos + v) % 10;
        p2_score += p2_pos + 1;
        rolls += 3;

        if p2_score >= 1000 {
            break;
        }
    }
    let (mut p1_wins, mut p2_wins) = (0, 0);
    for (roll, count) in [(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)] {
        let (p1, p2) = run_rec(true, 0, 0, p1_start - 1, p2_start - 1, roll);
        p1_wins += p1 * count;
        p2_wins += p2 * count;
    }
    (p1_score.min(p2_score) * rolls, p1_wins.max(p2_wins))
}

fn main() {
    println!("{:?}", run(4, 8));
    println!("{:?}", run(7, 9));
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_21() {
        let (first, second) = super::run(4, 8);
        assert_eq!(first, 679329);
        assert_eq!(second, 444356092776315);
    }

    #[test]
    fn test_input_21() {
        let (first, second) = super::run(7, 9);
        assert_eq!(first, 679329);
        assert_eq!(second, 433315766324816);
    }
}
