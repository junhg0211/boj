use std::io::stdin;
use std::cmp::max;

fn main() {
    // --- get metadata
    let (height, width, count) = {
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();

        let mut iter = input
            .trim()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap());

        (
            iter.next().unwrap(),
            iter.next().unwrap(),
            iter.next().unwrap(),
        )
    };

    // --- get board
    let mut board = vec![vec![false; width]; height];
    for _ in 0..count {
        let (r, c) = {
            let mut input = String::new();
            stdin().read_line(&mut input).unwrap();

            let mut iter = input
                .trim()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap());

            (
                iter.next().unwrap() - 1,
                iter.next().unwrap() - 1,
            )
        };

        board[r][c] = true;
    }

    // --- get biggest ssuregi
    let mut biggest = 0;
    for i in 0..height {
        for j in 0..width {
            if !board[i][j] {
                continue;
            }

            let mut queue = vec![(i, j)];
            board[i][j] = false;
            let mut size = 0;
            while !queue.is_empty() {
                let (y, x) = queue.pop().unwrap();
                size += 1;

                if y > 0 && board[y-1][x] {
                    queue.push((y-1, x));
                    board[y-1][x] = false;
                } if x > 0 && board[y][x-1] {
                    queue.push((y, x-1));
                    board[y][x-1] = false;
                } if y < height-1 && board[y+1][x] {
                    queue.push((y+1, x));
                    board[y+1][x] = false;
                } if x < width-1 && board[y][x+1] {
                    queue.push((y, x+1));
                    board[y][x+1] = false;
                }
            }

            biggest = max(biggest, size);
        }
    }

    // --- print result
    println!("{}", biggest);
}
