use std::io::stdin;

fn up(board: &Vec<Vec<u32>>, size: usize) -> Vec<Vec<u32>> {
    let mut board = board.clone();
    for i in 0..size {
        let mut blocks = Vec::new();
        for j in 0..size {
            if board[j][i] != 0 {
                blocks.push(board[j][i]);
            }
            board[j][i] = 0;
        }

        let mut j = 0;
        let mut k = 0;
        while j < blocks.len() {
            if j + 1 >= blocks.len() || blocks[j] != blocks[j + 1] {
                board[k][i] = blocks[j];
            } else {
                board[k][i] = blocks[j] * 2;
                j += 1;
            }
            k += 1;
            j += 1;
        }
    }

    return board;
}

fn down(board: &Vec<Vec<u32>>, size: usize) -> Vec<Vec<u32>> {
    let mut board = board.clone();
    for i in 0..size {
        let mut blocks = Vec::new();
        for j in 0..size {
            if board[size - 1 - j][i] != 0 {
                blocks.push(board[size - 1 - j][i]);
            }
            board[size - 1 - j][i] = 0;
        }

        let mut j = 0;
        let mut k = 0;
        while j < blocks.len() {
            if j + 1 >= blocks.len() || blocks[j] != blocks[j + 1] {
                board[size - 1 - k][i] = blocks[j];
            } else {
                board[size - 1 - k][i] = blocks[j] * 2;
                j += 1;
            }
            k += 1;
            j += 1;
        }
    }

    return board;
}

fn left(board: &Vec<Vec<u32>>, size: usize) -> Vec<Vec<u32>> {
    let mut board = board.clone();
    for i in 0..size {
        let mut blocks = Vec::new();
        for j in 0..size {
            if board[i][j] != 0 {
                blocks.push(board[i][j]);
            }
            board[i][j] = 0;
        }

        let mut j = 0;
        let mut k = 0;
        while j < blocks.len() {
            if j + 1 >= blocks.len() || blocks[j] != blocks[j + 1] {
                board[i][k] = blocks[j];
            } else {
                board[i][k] = blocks[j] * 2;
                j += 1;
            }
            k += 1;
            j += 1;
        }
    }

    return board;
}

fn right(board: &Vec<Vec<u32>>, size: usize) -> Vec<Vec<u32>> {
    let mut board = board.clone();
    for i in 0..size {
        let mut blocks = Vec::new();
        for j in 0..size {
            if board[i][size - 1 - j] != 0 {
                blocks.push(board[i][size - 1 - j]);
            }
            board[i][size - 1 - j] = 0;
        }

        let mut j = 0;
        let mut k = 0;
        while j < blocks.len() {
            if j + 1 >= blocks.len() || blocks[j] != blocks[j + 1] {
                board[i][size - 1 - k] = blocks[j];
            } else {
                board[i][size - 1 - k] = blocks[j] * 2;
                j += 1;
            }
            k += 1;
            j += 1;
        }
    }

    return board;
}

fn main() {
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let size = input.trim().parse::<usize>().unwrap();

    let mut board = Vec::new();
    for _ in 0..size {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let row = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        board.push(row);
    }

    let mut result = 0;
    for mut directions in 0..u32::pow(4, 6) {
        let mut clone = board.clone();

        for _ in 0..5 {
            directions >>= 2;
            if directions % 4 == 0 {
                clone = up(&clone, size);
            } else if directions % 4 == 1 {
                clone = left(&clone, size);
            } else if directions % 4 == 2 {
                clone = down(&clone, size);
            } else {
                clone = right(&clone, size);
            }
        }

        for i in 0..size {
            for j in 0..size {
                if result < clone[i][j] {
                    result = clone[i][j];
                }
            }
        }
    }

    println!("{}", result);
}
