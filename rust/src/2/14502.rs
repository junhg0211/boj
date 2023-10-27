use std::io::stdin;
use std::collections::VecDeque;

fn main() {
    // -- get board
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
            .split_whitespace()
            .map(|x| x.parse::<u32>().unwrap())
            .collect::<Vec<_>>();

        board.push(row);
    }

    let mut result = 0;
    for i1 in 0..(width*height) {
        let (y1, x1) = (i1 / width, i1 % width);
        if board[y1][x1] != 0 {
            continue;
        }
        board[y1][x1] = 1;

        for i2 in i1+1..(width*height) {
            let (y2, x2) = (i2 / width, i2 % width);
            if board[y2][x2] != 0 {
                continue;
            }
            board[y2][x2] = 1;

            for i3 in i2+1..(width*height) {
                let (y3, x3) = (i3 / width, i3 % width);
                if board[y3][x3] != 0 {
                    continue;
                }
                board[y3][x3] = 1;

                // clone board
                let mut petri = board.clone();

                // -- simulate infection
                for i in 0..height {
                    for j in 0..width {
                        if petri[i][j] != 2 {
                            continue;
                        }

                        let mut queue = VecDeque::new();
                        queue.push_back((i, j));

                        loop {
                            let (y, x) = match queue.pop_front() {
                                Some(thing) => thing,
                                None => break,
                            };

                            if y > 0 && petri[y-1][x] == 0 {
                                queue.push_back((y-1, x));
                                petri[y-1][x] = 2;
                            }
                            if x > 0 && petri[y][x-1] == 0 {
                                queue.push_back((y, x-1));
                                petri[y][x-1] = 2;
                            }
                            if y < height-1 && petri[y+1][x] == 0 {
                                queue.push_back((y+1, x));
                                petri[y+1][x] = 2;
                            }
                            if x < width-1 && petri[y][x+1] == 0 {
                                queue.push_back((y, x+1));
                                petri[y][x+1] = 2;
                            }
                        }
                    }
                }

                // -- count safe places
                let mut safe_count = 0;
                for i in 0..height {
                    for j in 0..width {
                        if petri[i][j] == 0 {
                            safe_count += 1;
                        }
                    }
                }

                if safe_count > result {
                    result = safe_count;
                }

                board[y3][x3] = 0;
            }
            board[y2][x2] = 0;
        }
        board[y1][x1] = 0;
    }

    println!("{}", result);
}
