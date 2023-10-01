use std::io::stdin;
use std::collections::VecDeque;

fn main() {
    // get width and height
    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();
    let mut iter = input.trim().split_whitespace();
    let height = iter.next().unwrap().parse::<usize>().unwrap();
    let width = iter.next().unwrap().parse::<usize>().unwrap();

    // board
    let mut board = Vec::new();
    for _ in 0..height {
        input.clear();
        stdin().read_line(&mut input).unwrap();
        board.push(input.trim().chars().collect::<Vec<char>>());
    }

    // calculate
    let mut been = vec![vec![false; width]; height];
    let mut result_sheeps = 0;
    let mut result_wolves = 0;
    for i in 0..height {
        for j in 0..width {
            if board[i][j] == '#' {
                continue;
            }

            if been[i][j] {
                continue;
            }

            let mut sheeps = 0;
            let mut wolves = 0;

            let mut queue = VecDeque::new();
            queue.push_back((j, i));
            been[i][j] = true;
            while !queue.is_empty() {
                let (x, y) = queue.pop_front().unwrap();

                match board[y][x] {
                    'v' => wolves += 1,
                    'o' => sheeps += 1,
                    _ => (),
                }

                if x < width-1 && !been[y][x+1] && board[y][x+1] != '#' {
                    queue.push_back((x+1, y));
                    been[y][x+1] = true;
                }

                if y < height-1 && !been[y+1][x] && board[y+1][x] != '#' {
                    queue.push_back((x, y+1));
                    been[y+1][x] = true;
                }

                if x > 0 && !been[y][x-1] && board[y][x-1] != '#' {
                    queue.push_back((x-1, y));
                    been[y][x-1] = true;
                }

                if y > 0 && !been[y-1][x] && board[y-1][x] != '#' {
                    queue.push_back((x, y-1));
                    been[y-1][x] = true;
                }
            }

            if sheeps > wolves {
                result_sheeps += sheeps;
            } else {
                result_wolves += wolves;
            }
        }
    }

    println!("{} {}", result_sheeps, result_wolves);
}
