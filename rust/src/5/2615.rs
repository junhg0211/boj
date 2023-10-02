use std::io::stdin;

fn main() {
    let mut board = Vec::new();
    for _ in 0..19 {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let row = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();
        board.push(row);
    }

    // vertically
    for i in 0+2..19-2 {
        for j in 0..19 {
            if board[i][j] == 0 {
                continue;
            }

            let mut wins = true;
            for k in -2..=2 {
                if board[(i as i32 + k) as usize][j] != board[i][j] {
                    wins = false;
                    break;
                }
            }

            if !wins {
                continue;
            }

            // check if 6
            if i >= 3 && board[i-3][j] == board[i][j]
                    || ((i+3) as usize) < 19 && board[i+3][j] == board[i][j] {
                continue;
            }

            println!("{}\n{} {}", board[i][j], i-2 + 1, j + 1);
            return;
        }
    }

    // horizontally
    for i in 0..19 {
        for j in 0+2..19-2 {
            if board[i][j] == 0 {
                continue;
            }

            let mut wins = true;
            for k in -2..=2 {
                if board[i][(j as i32 + k) as usize] != board[i][j] {
                    wins = false;
                    break;
                }
            }

            if !wins {
                continue;
            }

            // check if 6
            if j >= 3 && board[i][j-3] == board[i][j]
                    || ((j+3) as usize) < 19 && board[i][j+3] == board[i][j] {
                continue;
            }

            println!("{}\n{} {}", board[i][j], i + 1, j-2 + 1);
            return;
        }
    }

    // diagonally down
    for i in 0+2..19-2 {
        for j in 0+2..19-2 {
            if board[i][j] == 0 {
                continue;
            }

            let mut wins = true;
            for k in -2..=2 {
                if board[(i as i32 + k) as usize][(j as i32 + k) as usize] != board[i][j] {
                    wins = false;
                    break;
                }
            }

            if !wins {
                continue;
            }

            // check if 6
            if j >= 3 && i >= 3 && board[i-3][j-3] == board[i][j]
                    || (j+3) < 19 && (i+3) < 19 && board[i+3][j+3] == board[i][j] {
                continue;
            }

            println!("{}\n{} {}", board[i][j], i-2 + 1, j-2 + 1);
            return;
        }
    }

    // diagonally up
    for i in 0+2..19-2 {
        for j in 0+2..19-2 {
            if board[i][j] == 0 {
                continue;
            }

            let mut wins = true;
            for k in -2..=2 {
                if board[(i as i32 + k) as usize][(j as i32 - k) as usize] != board[i][j] {
                    wins = false;
                    break;
                }
            }

            if !wins {
                continue;
            }

            // check if 6
            if j >= 3 && (i+3) < 19 && board[i+3][j-3] == board[i][j]
                    || (j+3) < 19 && i >= 3 && board[i-3][j+3] == board[i][j] {
                continue;
            }

            println!("{}\n{} {}", board[i][j], i+2 + 1, j-2 + 1);
            return;
        }
    }

    println!("0");
}
