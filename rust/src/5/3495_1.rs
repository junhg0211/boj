use std::io::stdin;

fn main() {
    // -- get board size
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let height = iter.next().unwrap();
    let width = iter.next().unwrap();

    // -- get board
    let mut board = Vec::new();
    let mut half_count = 0;
    for _ in 0..height {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let row = input.trim().chars().collect::<Vec<_>>();

        for &letter in row.iter() {
            if letter == '.' {
                continue;
            }

            half_count += 1;
        }

        board.push(row);
    }

    // -- calculate area
    let mut area = 0;
    for i in 0..height {
        let mut inside = false;
        for j in 0..width {
            if board[i][j] == '/' || board[i][j] == '\\' {
                inside = !inside;
            }

            if inside && board[i][j] == '.' {
                area += 1;
            }
        }
    }

    println!("{}", area + half_count / 2);
}
