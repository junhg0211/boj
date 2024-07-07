use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let height = iter.next().unwrap();
    let width = iter.next().unwrap();
    let vertical = iter.next().unwrap();
    let horizontal = iter.next().unwrap();

    let mut board = Vec::new();
    for _ in 0..height {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let row = input.trim().chars().collect::<Vec<_>>();
        board.push(row);
    }

    let mut writer = BufWriter::new(stdout());
    for i in 0..height {
        for _ in 0..vertical {
            for j in 0..width {
                for _ in 0..horizontal {
                    write!(writer, "{}", board[i][j]).unwrap();
                }
            }
            writeln!(writer).unwrap();
        }
    }
}
