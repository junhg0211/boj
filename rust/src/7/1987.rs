use std::io::stdin;
use std::cmp::max;

fn f(
    y: usize, x: usize, height: usize, width: usize,
    board: &Vec<Vec<char>>,
    beens: &mut Vec<char>,
) -> usize {
    beens.push(board[y][x]);

    let mut result = beens.len();
    if y > 0 && !beens.contains(&board[y-1][x]) && result < 26 {
        result = max(result, f(y-1, x, height, width, board, beens));
    }
    if x > 0 && !beens.contains(&board[y][x-1]) && result < 26 {
        result = max(result, f(y, x-1, height, width, board, beens));
    }
    if y < height-1 && !beens.contains(&board[y+1][x]) && result < 26 {
        result = max(result, f(y+1, x, height, width, board, beens));
    }
    if x < width-1 && !beens.contains(&board[y][x+1]) && result < 26 {
        result = max(result, f(y, x+1, height, width, board, beens));
    }

    beens.pop().unwrap();

    return result;
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());
    let height = iter.next().unwrap();
    let width = iter.next().unwrap();

    let mut board = Vec::new();
    for _ in 0..height {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let row = input.trim().chars().collect::<Vec<_>>();
        board.push(row);
    }

    let result = f(0, 0, height, width, &board, &mut Vec::new());

    println!("{}", result);
}
