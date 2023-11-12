use std::io::stdin;
use std::collections::HashSet;

fn main() {
    let mut board = Vec::new();
    for _ in 0..9 {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let row = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        board.push(row);
    }

    let mut random = false;
    let mut wrongs = HashSet::<(usize, usize, u32)>::new();
    let mut track_stack = Vec::new();
    loop {
        let mut skipped_count = 0;
        let mut filled_count = 0;
        let mut skip = false;
        for i in 0..9 {
            for j in 0..9 {
                if board[i][j] != 0 {
                    continue;
                }

                let mut possibles = HashSet::new();
                for k in 1..=9 {
                    possibles.insert(k);
                }

                for k in 0..9 {
                    possibles.take(&board[i][k]);
                    possibles.take(&board[k][j]);

                    if wrongs.contains(&(i, j, k as u32)) {
                        possibles.take(&(k as u32));
                    }

                    let y = i/3*3 + k/3;
                    let x = j/3*3 + k%3;
                    possibles.take(&board[y][x]);
                }

                if possibles.len() == 0 {
                    println!("{:?}", track_stack.len());
                    let (y, x, number, then_board, then_wrongs) = track_stack.pop().unwrap();
                    wrongs = then_wrongs;
                    wrongs.insert((y, x, number));
                    board = then_board;
                    skip = true;
                    random = false;
                    break;
                } else if possibles.len() == 1 {
                    board[i][j] = possibles.drain().next().unwrap();
                    filled_count += 1;
                } else if random {
                    let value = possibles.drain().next().unwrap();
                    track_stack.push((i, j, value, board.clone(), wrongs.clone()));
                    board[i][j] = value;
                    random = false;
                    filled_count += 1;
                } else {
                    skipped_count += 1;
                }
            }

            if skip {
                break;
            }
        }

        if skip {
            continue;
        }

        if skipped_count == 0 {
            break;
        } else if filled_count == 0 {
            random = true;
        }
    }

    for row in board.iter() {
        for number in row.iter() {
            print!("{} ", number);
        }
        println!();
    }
}
