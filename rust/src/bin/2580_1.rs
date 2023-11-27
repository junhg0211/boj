use std::io::stdin;

fn remove(from: &mut Vec<usize>, obj: usize) {
    if let Some(index) = from.iter().position(|v| *v == obj) {
        from.swap_remove(index);
    }
}

fn print_board(board: &Vec<Vec<usize>>) {
    for row in board.iter() {
        for &letter in row.iter() {
            print!("{} ", letter);
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
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        for (j, &number) in row.iter().enumerate() {
            if number == 0 {
                zeros.push((i, j));
            }
        }

        board.push(row);
    }

    let mut assertions = Vec::new();
    let mut nos = vec![vec![false; 9]; zeros.len()];
    let mut index = 0;
    let mut possibles = Vec::new();
    while index < zeros.len() {
        let (i, j) = zeros[index];

        // -- get possibilities
        // remove possibles and fill with (1..=9)
        possibles.clear();
        for i in 1..=9 {
            possibles.push(i);
        }
        // get possibles
        for mut k in 0..9 {
            // remove from same row and column
            remove(&mut possibles, board[i][k]);
            remove(&mut possibles, board[k][j]);

            // remove from same subsquare
            let y = i/3*3 + k/3;
            let x = j/3*3 + k%3;
            remove(&mut possibles, board[y][x]);

            // remove from nos
            if nos[index][k] {
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
        for k in 0..9 {
            nos[index][k] = false;
        }

        // pop from stack
        let (new_index, thing) = assertions.pop().unwrap();
        let (y, x) = zeros[new_index];
        board[y][x] = 0;
        nos[new_index][thing-1] = true;
        index = new_index;
    }

    print_board(&board);
}
