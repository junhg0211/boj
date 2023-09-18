use std::io::stdin;
use std::collections::VecDeque;

fn blind_same(a: char, b: char) -> bool {
    a == b
        || a == 'R' && b == 'G'
        || b == 'R' && a == 'G'
}

fn main() {
    let size = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        input.trim().parse::<usize>().unwrap()
    };

    let mut board = Vec::new();
    for _ in 0..size {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        board.push(input.trim().chars().collect::<Vec<_>>());
    }
    let mut blind_been = vec![vec![false; size]; size];
    let mut not_blind_been = vec![vec![false; size]; size];

    let mut not_blind = 0;
    let mut blind = 0;
    for i in 0..size {
        for j in 0..size {
            if !blind_been[i][j] {
                let color = board[i][j];

                let mut queue = VecDeque::new();
                queue.push_back((i, j));
                blind += 1;
                blind_been[i][j] = true;

                while !queue.is_empty() {
                    let (y, x) = queue.pop_front().unwrap();

                    if y > 0 && !blind_been[y-1][x] && blind_same(board[y-1][x], board[y][x]) {
                        blind_been[y-1][x] = true;
                        queue.push_back((y-1, x));
                    }

                    if x > 0 && !blind_been[y][x-1] && blind_same(board[y][x-1], board[y][x]) {
                        blind_been[y][x-1] = true;
                        queue.push_back((y, x-1));
                    }

                    if y < size-1 && !blind_been[y+1][x] && blind_same(board[y+1][x], board[y][x]) {
                        blind_been[y+1][x] = true;
                        queue.push_back((y+1, x));
                    }

                    if x < size-1 && !blind_been[y][x+1] && blind_same(board[y][x+1], board[y][x]) {
                        blind_been[y][x+1] = true;
                        queue.push_back((y, x+1));
                    }
                }
            }

            if !not_blind_been[i][j] {
                let color = board[i][j];

                let mut queue = VecDeque::new();
                queue.push_back((i, j));
                not_blind += 1;
                not_blind_been[i][j] = true;

                while !queue.is_empty() {
                    let (y, x) = queue.pop_front().unwrap();

                    if y > 0 && !not_blind_been[y-1][x] && board[y-1][x] == board[y][x] {
                        not_blind_been[y-1][x] = true;
                        queue.push_back((y-1, x));
                    }

                    if x > 0 && !not_blind_been[y][x-1] && board[y][x-1] == board[y][x] {
                        not_blind_been[y][x-1] = true;
                        queue.push_back((y, x-1));
                    }

                    if y < size-1 && !not_blind_been[y+1][x] && board[y+1][x] == board[y][x] {
                        not_blind_been[y+1][x] = true;
                        queue.push_back((y+1, x));
                    }

                    if x < size-1 && !not_blind_been[y][x+1] && board[y][x+1] == board[y][x] {
                        not_blind_been[y][x+1] = true;
                        queue.push_back((y, x+1));
                    }
                }
            }
        }
    }

    println!("{} {}", not_blind, blind);
}
