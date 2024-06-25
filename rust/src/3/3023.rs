use std::io::{stdin, stdout, BufWriter, Write};

fn main() {
    // get width, height
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let height = iter.next().unwrap();
    let width = iter.next().unwrap();

    // get board
    let mut board = Vec::new();
    for _ in 0..height {
        // get row vector and push it to board
        input.clear();
        stdin().read_line(&mut input).unwrap();
        let row = input.trim().chars().collect::<Vec<_>>();
        board.push(row);
    }

    // get error position
    input.clear();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let y = iter.next().unwrap() - 1;
    let x = iter.next().unwrap() - 1;

    // print full board
    let mut writer = BufWriter::new(stdout());
    for i in 0..height * 2 {
        for j in 0..width * 2 {
            let is_reversed = y == i && x == j;

            // get index
            let i = if i >= height { height * 2 - i - 1 } else { i };
            let j = if j >= width { width * 2 - j - 1 } else { j };

            // get letter
            let letter = if is_reversed {
                if board[i][j] == '#' {
                    '.'
                } else {
                    '#'
                }
            } else {
                board[i][j]
            };

            // print
            write!(writer, "{}", letter).unwrap();
        }
        writeln!(writer).unwrap();
    }
}
