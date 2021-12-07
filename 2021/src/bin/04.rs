use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug)]
struct Board {
    width: usize,
    height: usize,
    tiles: Vec<usize>,
    won: bool,
}

impl Board {
    fn new(width: usize, height: usize, tiles: Vec<usize>) -> Board {
        Board {
            width,
            height,
            tiles,
            won: false,
        }
    }

    fn parse_board(reader: &mut BufReader<File>) -> Option<Board> {
        let mut width = 0;
        let mut tiles: Vec<usize> = vec![];
        for line in reader.lines() {
            let line = line.unwrap();
            let line = line.trim();
            if line.len() == 0 {
                return Some(Self::new(width, tiles.len() / width, tiles));
            }
            tiles.extend(line.split_whitespace().map(|s| s.parse::<usize>().unwrap()));
            if width == 0 {
                width = tiles.len();
            }
        }
        if tiles.len() > 0 {
            Some(Self::new(width, tiles.len() / width, tiles))
        } else {
            None
        }
    }

    fn is_win(&self, numbers: &Vec<usize>) -> bool {
        // horizontal
        for y in 0..self.height {
            if self.tiles[y * self.width..y * self.width + self.width]
                .iter()
                .all(|n| numbers.contains(n))
            {
                return true;
            }
        }
        // vertical
        for x in 0..self.width {
            let mut win = true;
            for y in 0..self.height {
                if !numbers.contains(&self.tiles[y * self.width + x]) {
                    win = false;
                    break;
                }
            }
            if win {
                return true;
            }
        }
        false
    }

    fn calc_score(&self, numbers: &Vec<usize>, last: usize) -> usize {
        self.tiles
            .iter()
            .map(|n| if numbers.contains(n) { 0 } else { *n })
            .sum::<usize>()
            * last
    }
}

fn run(filename: &str) -> (usize, usize) {
    let mut num_str = String::new();
    let file = File::open(filename).unwrap();
    let mut reader = BufReader::new(file);
    reader.read_line(&mut num_str).unwrap();
    let numbers: Vec<usize> = num_str
        .trim()
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect();
    // empty line
    reader.read_line(&mut num_str).unwrap();
    //println!("numbers: {:?}", numbers);
    let mut boards: Vec<Board> = vec![];
    while let Some(board) = Board::parse_board(&mut reader) {
        //println!("board {:?}", board);
        boards.push(board);
    }
    let mut drawn: Vec<usize> = vec![];
    let mut first: Option<usize> = None;
    let mut last: Option<usize> = None;
    for n in numbers {
        drawn.push(n);
        //println!("draw {}", n);
        for (i, board) in boards.iter_mut().enumerate() {
            if board.is_win(&drawn) && !board.won {
                let score = board.calc_score(&drawn, n);
                if first.is_none() {
                    first = Some(score);
                }
                last = Some(score);
                board.won = true;
                println!("board #{} wins with score {}", i + 1, score);
            }
        }
    }
    (first.unwrap(), last.unwrap())
}

fn main() {
    run("input/04-example.txt");
    run("input/04.txt");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_example_04() {
        let (first, second) = super::run("input/04-example.txt");
        assert_eq!(first, 4512);
        assert_eq!(second, 1924);
    }

    #[test]
    fn test_input_04() {
        let (first, second) = super::run("input/04.txt");
        assert_eq!(first, 50008);
        assert_eq!(second, 17408);
    }
}
