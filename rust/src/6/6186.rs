use std::io::stdin;
use std::collections::{ HashSet, VecDeque };

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
        let row = input
            .trim()
            .chars()
            .collect::<Vec<_>>();

        board.push(row);
    }

    let mut clump_count = 0;
    for i in 0..height {
        for j in 0..width {
            if board[i][j] == '.' {
                continue;
            }

            let mut stack = VecDeque::new();
            let mut beens = HashSet::new();
            stack.push_back((i, j));
            beens.insert((i, j));
            while let Some((y, x)) = stack.pop_front() {
                board[y][x] = '.';

                if y > 0 && board[y-1][x] == '#' && !beens.contains(&(y-1, x)) {
                    stack.push_back((y-1, x));
                    beens.insert((y-1, x));
                }

                if x > 0 && board[y][x-1] == '#' && !beens.contains(&(y, x-1)) {
                    stack.push_back((y, x-1));
                    beens.insert((y, x-1));
                }

                if y < height-1 && board[y+1][x] == '#' && !beens.contains(&(y+1, x)) {
                    stack.push_back((y+1, x));
                    beens.insert((y+1, x));
                }

                if x < width-1 && board[y][x+1] == '#' && !beens.contains(&(y, x+1)) {
                    stack.push_back((y, x+1));
                    beens.insert((y, x+1));
                }
            }

            clump_count += 1;
        }
    }

    println!("{}", clump_count);
}
