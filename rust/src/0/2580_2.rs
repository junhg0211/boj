use std::io::stdin;

type Board = [[u32; 9]; 9];

fn possible(y: usize, x: usize, board: &Board, this: u32) -> bool {
    for i in 0..9 {
        let yy = y / 3 * 3 + i / 3;
        let xx = x / 3 * 3 + i % 3;

        if board[y][i] == this {
            return false;
        }
        if board[i][x] == this {
            return false;
        }
        if board[yy][xx] == this {
            return false;
        }
    }

    return true;
}

fn answer(board: Board, mut position: usize) -> Option<Board> {
    while position < 81 {
        let i = position / 9;
        let j = position % 9;

        if board[i][j] != 0 {
            position += 1;
            continue;
        }

        for candidate in 1..=9 {
            if !possible(i, j, &board, candidate) {
                continue;
            }

            let mut new_board = board.clone();
            new_board[i][j] = candidate;
            let result = answer(new_board, position + 1);
            if let Some(_) = result {
                return result;
            }
        }

        return None;
    }

    return Some(board);
}

fn main() {
    let mut board = [[0; 9]; 9];

    for i in 0..9 {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        for (j, letter) in input.trim().split_whitespace().enumerate() {
            board[i][j] = letter.chars().next().unwrap() as u32 - '0' as u32;
        }
    }

    board = answer(board, 0).unwrap();

    for row in board {
        for letter in row {
            print!("{} ", letter);
        }
        println!();
    }
}
