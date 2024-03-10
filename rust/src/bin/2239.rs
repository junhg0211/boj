use std::collections::HashSet;
use std::io::stdin;

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
    let mut zeros = Vec::new();
    for i in 0..9 {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let row = input
            .trim()
            .chars()
            .map(|x| x as u32 - '0' as u32)
            .collect::<Vec<_>>();

        for (j, &number) in row.iter().enumerate() {
            if number == 0 {
                zeros.push((i, j));
            }
        }

        board.push(row);
    }

    let mut assertions = Vec::new();
    let mut nos = HashSet::new();
    let mut index = 0;
    let mut possibles = Vec::new();
    while index < zeros.len() {
        println!("{}", index);

        let (i, j) = zeros[index];

        // -- get possibilities
        // remove possibles and fill with (1..=9)
        possibles.clear();
        for i in 1..=9 {
            possibles.push(i);
        }
        // get possibles
        for k in 0..9 {
            // remove from same row and column
            remove(&mut possibles, board[i][k]);
            remove(&mut possibles, board[k][j]);

            // remove from same subsquare
            let y = i / 3 * 3 + k / 3;
            let x = j / 3 * 3 + k % 3;
            remove(&mut possibles, board[y][x]);

            // remove from nos
            let k = (k + 1) as u32;
            if nos.contains(&(index, k)) {
                remove(&mut possibles, k);
            }
        }
        possibles.sort();

        // -- if `possibles` is not empty, select one and push stack
        if possibles.len() > 0 {
            assertions.push((index, possibles[0]));
            board[i][j] = possibles[0];

            // process next loop
            index += 1;
            continue;
        }

        // -- if `possibles` is empty, pop stack and process next scenario
        // remove this nos
        for k in 1..=9 {
            nos.remove(&(index, k));
        }

        // pop from stack
        let (new_index, thing) = assertions.pop().unwrap();
        let (y, x) = zeros[new_index];
        board[y][x] = 0;
        nos.insert((new_index, thing));
        index = new_index;
    }

    print_board(&board);
}
