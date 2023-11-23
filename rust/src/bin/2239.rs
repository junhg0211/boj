use std::io::stdin;
use std::collections::HashSet;

fn remove(from: &mut Vec<u32>, obj: u32) {
    for i in 0..from.len() {
        if from[i] == obj {
            from.swap_remove(i);
            return;
        }
    }
}

fn print_board(board: &Vec<Vec<u32>>) {
    for row in board.iter() {
        for &letter in row.iter() {
            print!("{}", letter);
        }
        println!();
    }
}

fn main() {
    // -- get board
    let mut board = Vec::new();
    for _ in 0..9 {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let row = input
            .trim()
            .chars()
            .map(|x| x as u32 - '0' as u32)
            .collect::<Vec<_>>();
        board.push(row);
    }

    let mut assertions = Vec::new();
    let mut nos = HashSet::new();
    let mut i = 0;
    while i < 9 {
        let mut j = 0;
        while j < 9 {
            if board[i][j] != 0 {
                j += 1;
                continue;
            }

            // -- get possibilities
            let mut possibles = (1..=9).collect::<Vec<_>>();
            for k in 0..9 {
                // remove from same row and column
                remove(&mut possibles, board[i][k]);
                remove(&mut possibles, board[k][j]);

                // remove from same subsquare
                let y = i/3*3 + k/3;
                let x = j/3*3 + k%3;
                remove(&mut possibles, board[y][x]);

                // remove from nos
                if nos.contains(&(i, j, (k+1) as u32)) {
                    remove(&mut possibles, (k+1) as u32);
                }
            }
            possibles.sort();

            // if `possibles` is not empty, select one and push stack
            if possibles.len() > 0 {
                assertions.push((i, j, possibles[0]));
                board[i][j] = possibles[0];
            }
            // if `possibles` is empty, pop stack and process next scenario
            else {
                // remove this nos
                for k in 1..=9 {
                    nos.remove(&(i, j, k));
                }

                // pop from stack
                let (y, x, thing) = assertions.pop().unwrap();
                board[y][x] = 0;
                nos.insert((y, x, thing));
                (i, j) = (y, x);
                continue;
            }

            // process next loop
            j += 1;
        }
        i += 1;
    }

    print_board(&board);
}
