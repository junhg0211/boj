use std::io::stdin;

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

    // -- find path
    let mut stack = Vec::new();
    let mut result = 0;

    stack.push((0, 1, 0));
    // 0: horizontal, 1: vertical, 2: diagonal
    loop {
        let (y, x, p) = match stack.pop() {
            Some(thing) => thing,
            None => break,
        };

        if y == size-1 && x == size-1 {
            result += 1;
            continue;
        }

        // right
        if p != 1 && x < size-1 && board[y][x+1] == 0 {
            stack.push((y, x+1, 0));
        }
        // down
        if p != 0 && y < size-1 && board[y+1][x] == 0 {
            stack.push((y+1, x, 1));
        }
        // diagonal
        if x < size-1 && y < size-1
                && board[y][x+1] == 0
                && board[y+1][x] == 0
                && board[y+1][x+1] == 0 {
            stack.push((y+1, x+1, 2));
        }
    }

    println!("{}", result);
}
